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
}

pub fn run(project_file: &str, config_file: &str) -> Result<String, String> {
  println!("Deploying based on config file: {}", config_file);

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

  let branches = match config.branches {
    Some(v) => v,
    None => return Err("No branch configurations found".to_string()),
  };

  let branch_config = match branches.get(current_branch) {
    Some(v) => v,
    None => {
      return Ok(format!(
        "No branch configuration found for branch {}; no deployment necessary",
        current_branch
      ))
    }
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

  println!(
    "Found branch configuration for branch {}; deploying to experience {} and place {}",
    current_branch, experience_id, place_id
  );

  let mode = match branch_config.mode.as_ref().unwrap_or(&DeployMode::Publish) {
    DeployMode::Publish => DeployMode::Publish,
    DeployMode::Save => DeployMode::Save,
  };

  upload_place(project_file, experience_id, place_id, mode)
}
