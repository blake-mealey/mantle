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

    #[serde(default = "HashMap::new")]
    environments: HashMap<String, EnvironmentConfig>,

    #[serde(default = "HashMap::new")]
    branches: HashMap<String, BranchConfig>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnvironmentConfig {
    experience_id: Option<u64>,

    place_ids: Option<HashMap<String, u64>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BranchConfig {
    environment: Option<String>,

    deploy_mode: Option<DeployMode>,

    tag_commit: Option<bool>,
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

    let branch_config = match config.branches.get(current_branch) {
        Some(v) => v,
        None => {
            println!("✅ No branch configuration found; no deployment necessary");
            return Ok(());
        }
    };

    let environment_name = match &branch_config.environment {
        Some(v) => v,
        None => {
            return Err("Branch configuration does not contain an environment name.".to_string())
        }
    };

    let mode = match branch_config.deploy_mode.unwrap_or(DeployMode::Publish) {
        DeployMode::Publish => DeployMode::Publish,
        DeployMode::Save => DeployMode::Save,
    };

    let should_tag = branch_config.tag_commit.unwrap_or(false);

    println!("✅ Branch configuration:");
    println!("\tEnvironment: {}", environment_name);
    println!("\tDeploy mode: {}", mode);
    println!(
        "\tTag commit: {}",
        match should_tag {
            true => "Yes",
            false => "No",
        }
    );

    let environment_config = match config.environments.get(environment_name) {
        Some(v) => v,
        None => {
            return Err(format!(
                "No environment configuration found with name {}",
                environment_name
            ))
        }
    };

    let experience_id = match environment_config.experience_id {
        Some(v) => v,
        None => {
            return Err(format!(
                "No experience_id configuration found for branch {}",
                current_branch
            ))
        }
    };

    let place_ids = match &environment_config.place_ids {
        Some(v) => v,
        None => {
            return Err(format!(
                "No place_ids configuration found for branch {}.",
                current_branch
            ))
        }
    };

    println!("🌎 Environment configuration:");
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
        run_command("git push --tags").map_err(|e| format!("Unable to push the tag\n\t{}", e))?;
    }

    Ok(())
}
