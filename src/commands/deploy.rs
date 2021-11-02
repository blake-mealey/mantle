use crate::roblox_api::{
    DeployMode, ExperienceAnimationType, ExperienceAvatarType, ExperienceCollisionType,
    ExperienceConfigurationModel, ExperienceGenre, ExperiencePermissionsModel,
    ExperiencePlayableDevice, PlaceConfigurationModel, RobloxApi, SocialSlotType,
};
use crate::roblox_auth::RobloxAuth;
use serde::Deserialize;
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

//isFriendsOnly: true/false
//setActive(true/false)

#[derive(Deserialize)]
pub enum GenreConfig {
    All,
    Adventure,
    Building,
    Comedy,
    Fighting,
    FPS,
    Horror,
    Medieval,
    Military,
    Naval,
    RPG,
    SciFi,
    Sports,
    TownAndCity,
    Western,
}

#[derive(Deserialize)]
enum PlayabilityConfig {
    Private,
    Public,
    Friends,
}

#[derive(Deserialize)]
enum AvatarTypeConfig {
    R6,
    R15,
    PlayerChoice,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceTemplateConfig {
    // basic info
    genre: Option<GenreConfig>,
    playable_devices: Option<Vec<ExperiencePlayableDevice>>,
    // icon: Option<String>,   // TODO: call the upload icon api
    // thumbnails: Option<Vec<String>>  // TODO: call the upload thumbnails api

    // permissions
    playability: Option<PlayabilityConfig>, // TODO: call the setActive api for Private mode

    // monetization
    // badges: // TODO: create badges
    paid_access_price: Option<u32>,
    private_server_price: Option<u32>,
    // developer products: // TODO: create developer products

    // security
    enable_studio_access_to_apis: Option<bool>,
    allow_third_party_sales: Option<bool>,
    allow_third_party_teleports: Option<bool>,

    // localization: // TODO: localization

    // avatar
    avatar_type: Option<AvatarTypeConfig>,
    avatar_animation_type: Option<ExperienceAnimationType>,
    avatar_collision_type: Option<ExperienceCollisionType>,
    // avatar_asset_overrides: Option<HashMap<String, u64>>,    // TODO: figure out api
    // avatar_scale_constraints: Option<HashMap<String, (f32, f32)>>,   // TODO: figure out api

    // other
    // is_archived: Option<bool>,
}

impl From<ExperienceTemplateConfig> for ExperienceConfigurationModel {
    fn from(config: ExperienceTemplateConfig) -> Self {
        ExperienceConfigurationModel {
            genre: match config.genre {
                Some(GenreConfig::All) => Some(ExperienceGenre::All),
                Some(GenreConfig::Adventure) => Some(ExperienceGenre::Adventure),
                Some(GenreConfig::Building) => Some(ExperienceGenre::Tutorial),
                Some(GenreConfig::Comedy) => Some(ExperienceGenre::Funny),
                Some(GenreConfig::Fighting) => Some(ExperienceGenre::Ninja),
                Some(GenreConfig::FPS) => Some(ExperienceGenre::FPS),
                Some(GenreConfig::Horror) => Some(ExperienceGenre::Scary),
                Some(GenreConfig::Medieval) => Some(ExperienceGenre::Fantasy),
                Some(GenreConfig::Military) => Some(ExperienceGenre::War),
                Some(GenreConfig::Naval) => Some(ExperienceGenre::Pirate),
                Some(GenreConfig::RPG) => Some(ExperienceGenre::RPG),
                Some(GenreConfig::SciFi) => Some(ExperienceGenre::SciFi),
                Some(GenreConfig::Sports) => Some(ExperienceGenre::Sports),
                Some(GenreConfig::TownAndCity) => Some(ExperienceGenre::TownAndCity),
                Some(GenreConfig::Western) => Some(ExperienceGenre::WildWest),
                None => None,
            },
            playable_devices: config.playable_devices,

            is_friends_only: match config.playability {
                Some(PlayabilityConfig::Friends) => Some(true),
                Some(PlayabilityConfig::Public) => Some(false),
                _ => None,
            },

            is_for_sale: match config.paid_access_price {
                Some(_) => Some(true),
                _ => None,
            },
            price: config.paid_access_price,
            allow_private_servers: match config.private_server_price {
                Some(_) => Some(true),
                _ => None,
            },
            private_server_price: config.private_server_price,

            studio_access_to_apis_allowed: config.enable_studio_access_to_apis,
            permissions: match (
                config.allow_third_party_sales,
                config.allow_third_party_teleports,
            ) {
                (None, None) => None,
                (allow_third_party_sales, allow_third_party_teleports) => {
                    Some(ExperiencePermissionsModel {
                        is_third_party_purchase_allowed: allow_third_party_sales,
                        is_third_party_teleport_allowed: allow_third_party_teleports,
                    })
                }
            },

            universe_avatar_type: match config.avatar_type {
                Some(AvatarTypeConfig::R6) => Some(ExperienceAvatarType::MorphToR6),
                Some(AvatarTypeConfig::R15) => Some(ExperienceAvatarType::MorphToR15),
                Some(AvatarTypeConfig::PlayerChoice) => Some(ExperienceAvatarType::PlayerChoice),
                None => None,
            },
            universe_animation_type: config.avatar_animation_type,
            universe_collision_type: config.avatar_collision_type,
        }
    }
}

#[derive(Deserialize)]
enum ServerFillConfig {
    RobloxOptimized,
    Maximum,
    ReservedSlots(u32),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTemplateConfig {
    name: Option<String>,
    description: Option<String>,
    max_player_count: Option<u32>,
    allow_copying: Option<bool>,
    server_fill: Option<ServerFillConfig>,
}

impl From<&PlaceTemplateConfig> for PlaceConfigurationModel {
    fn from(config: &PlaceTemplateConfig) -> Self {
        PlaceConfigurationModel {
            name: config.name.clone(),
            description: config.description.clone(),
            max_player_count: config.max_player_count,
            allow_copying: config.allow_copying,
            social_slot_type: match config.server_fill {
                Some(ServerFillConfig::RobloxOptimized) => Some(SocialSlotType::Automatic),
                Some(ServerFillConfig::Maximum) => Some(SocialSlotType::Empty),
                Some(ServerFillConfig::ReservedSlots(_)) => Some(SocialSlotType::Custom),
                None => None,
            },
            custom_social_slot_count: match config.server_fill {
                Some(ServerFillConfig::ReservedSlots(count)) => Some(count),
                _ => None,
            },
        }
    }
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
        roblox_api.configure_experience(experience_id, &experience_template.unwrap().into())?;
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
            roblox_api.configure_place(*place_id, &place_template.unwrap().into())?;
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
