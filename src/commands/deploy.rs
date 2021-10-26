use crate::roblox_api::{upload_place, DeployMode};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::str;

#[derive(Deserialize)]
struct Config {
    place_file: Option<String>,

    #[serde(default = "HashMap::new")]
    environments: HashMap<String, EnvironmentConfig>,

    #[serde(default = "HashMap::new")]
    branches: HashMap<String, BranchConfig>,
}

#[derive(Deserialize)]
struct EnvironmentConfig {
    experience_id: Option<u64>,

    place_id: Option<u64>,
}

#[derive(Deserialize)]
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

pub fn run(config_file: &str) -> Result<String, String> {
    println!("📃 Config file: {}", config_file);

    let data = match fs::read_to_string(config_file) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!(
                "Unable to read config file: {}\n\t{}",
                config_file, e
            ))
        }
    };

    let config: Config = match toml::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!(
                "Unable to parse config file {}\n\t{}",
                config_file, e
            ))
        }
    };

    let place_file = match config.place_file {
        Some(v) => v,
        None => return Err("No place file found in configuration".to_string()),
    };

    println!("📁 Project file: {}", place_file);

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
        None => return Ok("✅ No branch configuration found; no deployment necessary".to_string()),
    };

    let environment_name = match &branch_config.environment {
        Some(v) => v,
        None => {
            return Err("Branch configuration does not contain an environment name.".to_string())
        }
    };

    let mode = match branch_config
        .deploy_mode
        .as_ref()
        .unwrap_or(&DeployMode::Publish)
    {
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

    let place_id = match environment_config.place_id {
        Some(v) => v,
        None => {
            return Err(format!(
                "No place_id configuration found for branch {}",
                current_branch
            ))
        }
    };

    println!("✅ Environment configuration:");
    println!("\tExperience ID: {}", experience_id);
    println!("\tPlace ID: {}", place_id);

    let result = upload_place(&place_file, experience_id, place_id, mode)?;

    if should_tag {
        let tag = format!("v{}", result.place_version);
        println!("🔖 Tagging commit with: {}", tag);

        let tag_output = run_command(&format!("git tag {}", tag));
        if tag_output.is_err() {
            return Err(format!(
                "Unable to tag the current commit\n\t{}",
                tag_output.unwrap_err()
            ));
        }
        let push_output = run_command("git push --tags");
        if push_output.is_err() {
            return Err(format!(
                "Unable to push the tag\n\t{}",
                tag_output.unwrap_err()
            ));
        }
    }

    Ok(result.message)
}
