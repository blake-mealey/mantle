extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};
use std::fs;
use std::str;
use std::collections::HashMap;
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    file: Option<String>,
    branches: Option<HashMap<String, BranchConfig>>
}

#[derive(Deserialize)]
enum BranchMode {
    Publish,
    Save
}

#[derive(Deserialize)]
struct BranchConfig {
    experience_id: Option<u64>,
    place_id: Option<u64>,
    mode: Option<BranchMode>
}

fn command_save(project_file: &str, experience_id: u64, place_id: u64) {
    println!("save {} to {} > {}", project_file, experience_id, place_id);
}

fn command_publish(project_file: &str, experience_id: u64, place_id: u64) {
    println!("publish {} to {} > {}", project_file, experience_id, place_id);
}

fn command_deploy(config_file: &str) {
    println!("Deploying based on config file {}", config_file);

    let data = fs::read_to_string(config_file).expect("Unable to read file.");
    let config: Config = toml::from_str(&data).expect("Unable to parse file.");

    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .arg("/C")
                .arg("git symbolic-ref --short HEAD")
                .output()
                .expect("Unable to check git branch.")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("git symbolic-ref --short HEAD")
                .output()
                .expect("Unable to check git branch.")
    };

    let current_branch = str::from_utf8(&result.stdout).unwrap().trim();

    let branches = config.branches.expect("No branches configuration");

    let branch_config = branches.get(current_branch);
    if branch_config.is_none() {
        println!("No config for current branch {}", current_branch);
        return;
    }

    println!("Deploying with the config for branch {}", current_branch);

    let mode = branch_config.unwrap().mode.as_ref();
    match mode.unwrap_or(&BranchMode::Publish) {
        BranchMode::Publish => command_publish(
            &config.file.unwrap(),
            branch_config.unwrap().experience_id.unwrap(),
            branch_config.unwrap().place_id.unwrap()
        ),
        BranchMode::Save => command_save(
            &config.file.unwrap(),
            branch_config.unwrap().experience_id.unwrap(),
            branch_config.unwrap().place_id.unwrap()
        )
    }
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
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom deploy config file")
                .default_value("rocat.toml")
                .takes_value(true)))
        .get_matches();

    match matches.subcommand() {
        ("save", Some(save_matches)) => {
            command_save(
                save_matches.value_of("FILE").unwrap(),
                save_matches.value_of("EXPERIENCE_ID").unwrap().parse::<u64>().unwrap(),
                save_matches.value_of("PLACE_ID").unwrap().parse::<u64>().unwrap()
            );
        }
        ("publish", Some(publish_matches)) => {
            command_publish(
                publish_matches.value_of("FILE").unwrap(),
                publish_matches.value_of("EXPERIENCE_ID").unwrap().parse::<u64>().unwrap(),
                publish_matches.value_of("PLACE_ID").unwrap().parse::<u64>().unwrap()
            );
        }
        ("deploy", Some(deploy_matches)) => {
            command_deploy(
                deploy_matches.value_of("config").unwrap()
            );
        }
        _ => {}
    }
}
