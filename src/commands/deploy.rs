use crate::roblox_api::{upload_place, DeployMode};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::str;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    place_files: Option<HashMap<String, String>>,

    #[serde(default = "Vec::new")]
    deployments: Vec<DeploymentConfig>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeploymentConfig {
    name: Option<String>,

    #[serde(default = "Vec::new")]
    branches: Vec<String>,

    deploy_mode: Option<DeployMode>,

    tag_commit: Option<bool>,

    experience_id: Option<u64>,

    place_ids: Option<HashMap<String, u64>>,
}

fn run_command(command: &str) -> std::io::Result<std::process::Output> {
    if cfg!(target_os = "windows") {
        return Command::new("cmd").arg("/C").arg(command).output();
    } else {
        return Command::new("sh").arg("-c").arg(command).output();
    }
}

fn load_config_file(config_file: &str) -> Result<Config, String> {
    let data = match fs::read_to_string(config_file) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!(
                "Unable to read config file: {}\n\t{}",
                config_file, e
            ))
        }
    };

    match serde_yaml::from_str::<Config>(&data) {
        Ok(v) => Ok(v),
        Err(e) => {
            return Err(format!(
                "Unable to parse config file {}\n\t{}",
                config_file, e
            ))
        }
    }
}

fn match_branch(branch: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        let glob_pattern = glob::Pattern::new(pattern);
        if glob_pattern.is_ok() && glob_pattern.unwrap().matches(branch) {
            return true;
        }
    }
    false
}

pub fn run(config_file: &str) -> Result<(), String> {
    println!("📃 Config file: {}", config_file);
    let config = load_config_file(config_file)?;

    let output = run_command("git symbolic-ref --short HEAD");
    let result = match output {
        Ok(v) => v,
        Err(e) => {
            return Err(format!(
                "Unable to determine git branch. Are you in a git repository?\n\t{}",
                e
            ))
        }
    };

    if !result.status.success() {
        return Err("Unable to determine git branch. Are you in a git repository?".to_string());
    }

    let current_branch = str::from_utf8(&result.stdout).unwrap().trim();
    if current_branch.is_empty() {
        return Err("Unable to determine git branch. Are you in a git repository?".to_string());
    }

    println!("🌿 Git branch: {}", current_branch);

    let deployment = config
        .deployments
        .iter()
        .find(|deployment| match_branch(current_branch, &deployment.branches));

    let deployment = match deployment {
        Some(v) => v,
        None => {
            println!("✅ No deployment configuration found for branch; no deployment necessary.");
            return Ok(());
        }
    };

    let deployment_name = match &deployment.name {
        Some(v) => v,
        None => return Err("Deployment configuration does not contain a name.".to_string()),
    };

    let mode = match deployment.deploy_mode.unwrap_or(DeployMode::Publish) {
        DeployMode::Publish => DeployMode::Publish,
        DeployMode::Save => DeployMode::Save,
    };

    let should_tag = deployment.tag_commit.unwrap_or(false);

    let experience_id = match deployment.experience_id {
        Some(v) => v,
        None => {
            return Err(format!(
                "No experience_id configuration found for branch {}",
                current_branch
            ))
        }
    };

    let place_ids = match &deployment.place_ids {
        Some(v) => v,
        None => {
            return Err(format!(
                "No place_ids configuration found for branch {}.",
                current_branch
            ))
        }
    };

    println!("🌎 Deployment configuration:");
    println!("\tName: {}", deployment_name);
    println!("\tDeploy mode: {}", mode);
    println!(
        "\tTag commit: {}",
        match should_tag {
            true => "Yes",
            false => "No",
        }
    );
    println!("\tExperience ID: {}", experience_id);
    println!("\tPlace IDs:");
    for (name, place_id) in place_ids.iter() {
        println!("\t\t{}: {}", name, place_id);
    }

    let place_files = config.place_files.unwrap();
    for (name, place_file) in place_files.iter() {
        println!("🚀 Deploying place: {}", name);

        let place_id = match place_ids.get(name) {
            Some(v) => v,
            None => return Err(format!("No place ID found for configured place {}", name)),
        };

        let result = upload_place(place_file, experience_id, *place_id, mode)?;

        if should_tag {
            let tag = format!("{}-v{}", name, result.place_version);
            println!("🔖 Tagging commit with: {}", tag);

            run_command(&format!("git tag {}", tag))
                .map_err(|e| format!("Unable to tag the current commit\n\t{}", e))?;
        }
    }

    if should_tag {
        run_command("git push --tags").map_err(|e| format!("Unable to push the tags\n\t{}", e))?;
    }

    Ok(())
}
