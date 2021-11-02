use crate::roblox_api::{DeployMode, RobloxApi};
use crate::roblox_auth::RobloxAuth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::str;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    #[serde(default = "HashMap::new")]
    place_files: HashMap<String, String>,

    #[serde(default = "Vec::new")]
    deployments: Vec<DeploymentConfig>,

    #[serde(default)]
    templates: TemplateConfig,
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

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct TemplateConfig {
    experience: Option<ExperienceTemplateConfig>,

    #[serde(default = "HashMap::new")]
    places: HashMap<String, PlaceTemplateConfig>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceTemplateConfig {
    name: Option<String>,
    allow_private_servers: Option<bool>,
    private_server_price: Option<i32>,
    universe_avatar_type: Option<String>,           // TODO: enum
    universe_animation_type: Option<String>,        // TODO: enum
    universe_collision_type: Option<String>,        // TODO: enum
    universe_join_positioning_type: Option<String>, // TODO: enum
    is_archived: Option<bool>,
    is_friends_only: Option<bool>,
    genre: Option<String>,                 //TODO: enum
    playable_devices: Option<Vec<String>>, //TODO: enum
    is_for_sale: Option<bool>,
    price: Option<i32>,
    // universe_avatar_asset_overrides: Option<Vec<unknown>>,
    universe_avatar_min_scales: Option<HashMap<String, f32>>, // TODO: enum
    universe_avatar_max_scales: Option<HashMap<String, f32>>, // TODO: enum
    studio_access_to_apis_allowed: Option<bool>,
    // permissions: Option<ExperienceTemplatePermissionsConfig>, // TODO: struct
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTemplateConfig {
    name: Option<String>,
    description: Option<String>,
    max_player_count: Option<i32>,
    allow_copying: Option<bool>,
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

    let mut roblox_api = RobloxApi::new(RobloxAuth::new());

    let experience_template = config.templates.experience;
    if experience_template.is_some() {
        println!("🔧 Configuring experience");
        roblox_api.configure_experience(experience_id, &experience_template.unwrap())?;
    }

    for (name, place_file) in config.place_files.iter() {
        println!("🚀 Deploying place: {}", name);

        let place_id = match place_ids.get(name) {
            Some(v) => v,
            None => return Err(format!("No place ID found for configured place {}", name)),
        };

        let place_template = config.templates.places.get(name);
        if place_template.is_some() {
            println!("\t🔧 Configuring place");
            roblox_api.configure_place(*place_id, &place_template.unwrap())?;
        }

        let upload_result = roblox_api.upload_place(place_file, experience_id, *place_id, mode)?;

        if should_tag {
            let tag = format!("{}-v{}", name, upload_result.place_version);
            println!("\t🔖 Tagging commit with: {}", tag);

            run_command(&format!("git tag {}", tag))
                .map_err(|e| format!("Unable to tag the current commit\n\t{}", e))?;
        }
    }

    if should_tag {
        run_command("git push --tags").map_err(|e| format!("Unable to push the tags\n\t{}", e))?;
    }

    Ok(())
}
