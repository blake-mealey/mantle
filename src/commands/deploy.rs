use crate::roblox_api::{upload_place, DeployMode};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::str;

#[derive(Deserialize)]
struct Config {
    branches: Option<HashMap<String, BranchConfig>>,
}

#[derive(Deserialize)]
struct BranchConfig {
    experience_id: Option<u64>,
    place_id: Option<u64>,
    mode: Option<DeployMode>,
    tag: Option<bool>,
}

fn run_command(command: &str) -> std::io::Result<std::process::Output> {
    if cfg!(target_os = "windows") {
        return Command::new("cmd").arg("/C").arg(command).output();
    } else {
        return Command::new("sh").arg("-c").arg(command).output();
    }
}

pub fn run(project_file: &str, config_file: &str) -> Result<String, String> {
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

    let branches = match config.branches {
        Some(v) => v,
        None => return Err("No branch configurations found".to_string()),
    };

    let branch_config = match branches.get(current_branch) {
        Some(v) => v,
        None => return Ok("✅ No branch configuration found; no deployment necessary".to_string()),
    };

    let experience_id = match branch_config.experience_id {
        Some(v) => v,
        None => {
            return Err(format!(
                "No experience_id configuration found for branch {}",
                current_branch
            ))
        }
    };

    let place_id = match branch_config.place_id {
        Some(v) => v,
        None => {
            return Err(format!(
                "No place_id configuration found for branch {}",
                current_branch
            ))
        }
    };

    let mode = match branch_config.mode.as_ref().unwrap_or(&DeployMode::Publish) {
        DeployMode::Publish => DeployMode::Publish,
        DeployMode::Save => DeployMode::Save,
    };

    let should_tag = branch_config.tag.unwrap_or(false);

    println!("✅ Branch configuration:");
    println!("\tExperience ID: {}", experience_id);
    println!("\tPlace ID: {}", place_id);
    println!("\tDeploy mode: {}", mode);
    println!(
        "\tTag commit: {}",
        match should_tag {
            true => "Yes",
            false => "No",
        }
    );

    let result = upload_place(project_file, experience_id, place_id, mode)?;

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
