extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};
use std::path::Path;
use std::ffi::OsStr;
use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    branches: Option<HashMap<String, BranchConfig>>
}

#[derive(Deserialize)]
enum DeployMode {
    Publish,
    Save
}

#[derive(Deserialize)]
struct BranchConfig {
    experience_id: Option<u64>,
    place_id: Option<u64>,
    mode: Option<DeployMode>
}

enum ProjectType {
    Xml,
    Binary
}

fn upload_place(project_file: &str, experience_id: u64, place_id: u64, mode: DeployMode) -> Result<String, String> {
    let api_key = &match env::var("ROBLOX_API_KEY") {
        Ok(val) => val,
        Err(_) => return Err(format!("ROBLOX_API_KEY environment variable not set"))
    };

    let project_type = match Path::new(project_file).extension().and_then(OsStr::to_str) {
        Some("rbxlx") => ProjectType::Xml,
        Some("rbxl") => ProjectType::Binary,
        Some(v) => return Err(format!("Invalid project file extension: {}", v)),
        None => return Err(format!("No project file extension in project file: {}", project_file))
    };

    let content_type = match project_type {
        ProjectType::Xml => "application/xml",
        ProjectType::Binary => "application/octet-stream"
    };

    let version_type = match mode {
        DeployMode::Publish => "Published",
        DeployMode::Save => "Saved"
    };

    let req = ureq::post(&format!("https://apis.roblox.com/universes/v1/{}/places/{}/versions", experience_id, place_id))
        .set("x-api-key", api_key)
        .set("Content-Type", content_type)
        .query("versionType", version_type);

    let res = match project_type {
        ProjectType::Xml => {
            let data = match fs::read_to_string(project_file) {
                Ok(v) => v,
                Err(e) => return Err(format!("Unable to read project file: {}\n\t{}", project_file, e))
            };
            req.send_string(&data)
        },
        ProjectType::Binary => {
            let data = match fs::read(project_file) {
                Ok(v) => v,
                Err(e) => return Err(format!("Unable to read project file: {}\n\t{}", project_file, e))
            };
            req.send_bytes(&data)
        }
    };

    return match res {
        Ok(response) => Ok(response.into_string().unwrap()),
        Err(ureq::Error::Status(_code, response)) => match response.status() {
            400 => Err(format!("Invalid request or file content: {}", response.into_string().unwrap())),
            401 => Err(format!("API key not valid for operation: {}", response.into_string().unwrap())),
            403 => Err(format!("Publish not allowed on place: {}", response.into_string().unwrap())),
            404 => Err(format!("Place or universe does not exist: {}", response.into_string().unwrap())),
            409 => Err(format!("Place not part of the universe: {}", response.into_string().unwrap())),
            500 => Err(format!("Server internal error: {}", response.into_string().unwrap())),
            status => Err(format!("Unknown error (status {}): {}", status, response.into_string().unwrap()))
        },
        Err(e) => Err(format!("Generic error: {}", e))
    }
}

fn command_save(project_file: &str, experience_id: &str, place_id: &str) -> Result<String, String> {
    let parsed_experience_id = match experience_id.parse::<u64>() {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid EXPERIENCE_ID: {}\n\t{}", experience_id, e))
    };

    let parsed_place_id = match place_id.parse::<u64>() {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid PLACE_ID: {}\n\t{}", place_id, e))
    };

    return upload_place(project_file, parsed_experience_id, parsed_place_id, DeployMode::Save);
}

fn command_publish(project_file: &str, experience_id: &str, place_id: &str) -> Result<String, String> {
    let parsed_experience_id = match experience_id.parse::<u64>() {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid EXPERIENCE_ID: {}\n\t{}", experience_id, e))
    };

    let parsed_place_id = match place_id.parse::<u64>() {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid PLACE_ID: {}\n\t{}", place_id, e))
    };

    return upload_place(project_file, parsed_experience_id, parsed_place_id, DeployMode::Publish);
}

fn command_deploy(project_file: &str, config_file: &str) -> Result<String, String> {
    println!("Deploying based on config file: {}", config_file);

    let data = match fs::read_to_string(config_file) {
        Ok(v) => v,
        Err(e) => return Err(format!("Unable to read config file: {}\n\t{}", config_file, e))
    };

    let config: Config = match toml::from_str(&data) {
        Ok(v) => v,
        Err(e) => return Err(format!("Unable to parse config file {}\n\t{}", config_file, e))
    };

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .arg("/C")
                .arg("git symbolic-ref --short HEAD")
                .output()
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("git symbolic-ref --short HEAD")
                .output()
    };

    let result = match output {
        Ok(v) => v,
        Err(e) => return Err(format!("Unable to determine git branch. Are you in a git repository?\n\t{}", e))
    };

    if !result.status.success() {
        return Err(format!("Unable to determine git branch. Are you in a git repository?"));
    }

    let current_branch = str::from_utf8(&result.stdout).unwrap().trim();
    if current_branch.is_empty() {
        return Err(format!("Unable to determine git branch. Are you in a git repository?"));
    }

    println!("Operating on branch: {}", current_branch);

    let branches = match config.branches {
        Some(v) => v,
        None => return Err(format!("No branch configurations found"))
    };

    let branch_config = match branches.get(current_branch) {
        Some(v) => v,
        None => return Ok(format!("No branch configuration found for branch; no deployment necessary"))
    };

    let mode = match branch_config.mode.as_ref().unwrap_or(&DeployMode::Publish) {
        &DeployMode::Publish => DeployMode::Publish,
        &DeployMode::Save => DeployMode::Save
    };

    return upload_place(
        project_file,
        branch_config.experience_id.unwrap(),
        branch_config.place_id.unwrap(),
        mode
    );
}

fn main() {
    let matches = App::new("rocat")
        .version("0.1.0")
        .about("Manages Roblox deployments")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("save")
            .about("Saves a project file to a Roblox place")
            .arg(Arg::with_name("FILE")
                .required(true)
                .index(1)
                .takes_value(true))
            .arg(Arg::with_name("EXPERIENCE_ID")
                .required(true)
                .index(2)
                .takes_value(true))
            .arg(Arg::with_name("PLACE_ID")
                .required(true)
                .index(3)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("publish")
            .about("Publishes a project file to a Roblox place")
            .arg(Arg::with_name("FILE")
                .required(true)
                .index(1)
                .takes_value(true))
            .arg(Arg::with_name("EXPERIENCE_ID")
                .required(true)
                .index(2)
                .takes_value(true))
            .arg(Arg::with_name("PLACE_ID")
                .required(true)
                .index(3)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("deploy")
            .about("Saves a project file to a Roblox place")
            .arg(Arg::with_name("FILE")
                .required(true)
                .index(1)
                .takes_value(true))
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom deploy config file")
                .default_value("rocat.toml")
                .takes_value(true)))
        .get_matches();

    let result = match matches.subcommand() {
        ("save", Some(save_matches)) => {
            command_save(
                save_matches.value_of("FILE").unwrap(),
                save_matches.value_of("EXPERIENCE_ID").unwrap(),
                save_matches.value_of("PLACE_ID").unwrap()
            )
        }
        ("publish", Some(publish_matches)) => {
            command_publish(
                publish_matches.value_of("FILE").unwrap(),
                publish_matches.value_of("EXPERIENCE_ID").unwrap(),
                publish_matches.value_of("PLACE_ID").unwrap()
            )
        }
        ("deploy", Some(deploy_matches)) => {
            command_deploy(
                deploy_matches.value_of("FILE").unwrap(),
                deploy_matches.value_of("config").unwrap()
            )
        }
        _ => Err("Unreachable code reached!".to_string())
    };

    std::process::exit(match result {
        Ok(v) => {
            println!("{}", v);
            0
        },
        Err(e) => {
            println!("{}", e);
            1
        }
    });
}
