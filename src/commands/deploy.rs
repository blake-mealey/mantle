use crate::{
    resource_manager::{resource_types, AssetId, RobloxResourceManager, SINGLETON_RESOURCE_ID},
    resources::{InputRef, Resource, ResourceGraph, ResourceManager},
    roblox_api::{
        DeployMode, ExperienceAnimationType, ExperienceAvatarType, ExperienceCollisionType,
        ExperienceConfigurationModel, ExperienceGenre, ExperiencePermissionsModel,
        ExperiencePlayableDevice, PlaceConfigurationModel, SocialSlotType,
    },
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    str,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default = "HashMap::new")]
    pub place_files: HashMap<String, String>,

    #[serde(default = "Vec::new")]
    pub deployments: Vec<DeploymentConfig>,

    #[serde(default)]
    pub templates: TemplateConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentConfig {
    pub name: String,

    #[serde(default = "Vec::new")]
    pub branches: Vec<String>,

    #[serde(default)]
    pub deploy_mode: DeployMode,

    #[serde(default)]
    pub tag_commit: bool,

    pub experience_id: u64,

    pub place_ids: HashMap<String, u64>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TemplateConfig {
    pub experience: Option<ExperienceTemplateConfig>,

    #[serde(default = "HashMap::new")]
    pub places: HashMap<String, PlaceTemplateConfig>,
}

//isFriendsOnly: true/false
//setActive(true/false)

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GenreConfig {
    All,
    Adventure,
    Building,
    Comedy,
    Fighting,
    Fps,
    Horror,
    Medieval,
    Military,
    Naval,
    Rpg,
    SciFi,
    Sports,
    TownAndCity,
    Western,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayabilityConfig {
    Private,
    Public,
    Friends,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AvatarTypeConfig {
    R6,
    R15,
    PlayerChoice,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceTemplateConfig {
    // basic info
    pub genre: Option<GenreConfig>,
    pub playable_devices: Option<Vec<ExperiencePlayableDevice>>,
    pub icon: Option<String>,
    pub thumbnails: Option<Vec<String>>,

    // permissions
    pub playability: Option<PlayabilityConfig>,

    // monetization
    // badges: // TODO: create badges
    pub paid_access_price: Option<u32>,
    pub private_server_price: Option<u32>,
    // developer products: // TODO: create developer products

    // security
    pub enable_studio_access_to_apis: Option<bool>,
    pub allow_third_party_sales: Option<bool>,
    pub allow_third_party_teleports: Option<bool>,

    // localization: // TODO: localization

    // avatar
    pub avatar_type: Option<AvatarTypeConfig>,
    pub avatar_animation_type: Option<ExperienceAnimationType>,
    pub avatar_collision_type: Option<ExperienceCollisionType>,
    // avatar_asset_overrides: Option<HashMap<String, u64>>,    // TODO: figure out api
    // avatar_scale_constraints: Option<HashMap<String, (f32, f32)>>,   // TODO: figure out api

    // other
    // is_archived: Option<bool>,
}

impl From<&ExperienceTemplateConfig> for ExperienceConfigurationModel {
    fn from(config: &ExperienceTemplateConfig) -> Self {
        ExperienceConfigurationModel {
            genre: match config.genre {
                Some(GenreConfig::All) => Some(ExperienceGenre::All),
                Some(GenreConfig::Adventure) => Some(ExperienceGenre::Adventure),
                Some(GenreConfig::Building) => Some(ExperienceGenre::Tutorial),
                Some(GenreConfig::Comedy) => Some(ExperienceGenre::Funny),
                Some(GenreConfig::Fighting) => Some(ExperienceGenre::Ninja),
                Some(GenreConfig::Fps) => Some(ExperienceGenre::Fps),
                Some(GenreConfig::Horror) => Some(ExperienceGenre::Scary),
                Some(GenreConfig::Medieval) => Some(ExperienceGenre::Fantasy),
                Some(GenreConfig::Military) => Some(ExperienceGenre::War),
                Some(GenreConfig::Naval) => Some(ExperienceGenre::Pirate),
                Some(GenreConfig::Rpg) => Some(ExperienceGenre::Rpg),
                Some(GenreConfig::SciFi) => Some(ExperienceGenre::SciFi),
                Some(GenreConfig::Sports) => Some(ExperienceGenre::Sports),
                Some(GenreConfig::TownAndCity) => Some(ExperienceGenre::TownAndCity),
                Some(GenreConfig::Western) => Some(ExperienceGenre::WildWest),
                None => None,
            },
            playable_devices: config
                .playable_devices
                .as_ref()
                .map(|devices| devices.to_vec()),

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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ServerFillConfig {
    RobloxOptimized,
    Maximum,
    ReservedSlots(u32),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTemplateConfig {
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_player_count: Option<u32>,
    pub allow_copying: Option<bool>,
    pub server_fill: Option<ServerFillConfig>,
}

impl From<PlaceTemplateConfig> for PlaceConfigurationModel {
    fn from(config: PlaceTemplateConfig) -> Self {
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

fn load_config_file(config_file: &Path) -> Result<Config, String> {
    let data = match fs::read_to_string(config_file) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!(
                "Unable to read config file: {}\n\t{}",
                config_file.display(),
                e
            ))
        }
    };

    match serde_yaml::from_str::<Config>(&data) {
        Ok(v) => Ok(v),
        Err(e) => {
            return Err(format!(
                "Unable to parse config file {}\n\t{}",
                config_file.display(),
                e
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

fn parse_project(project: Option<&str>) -> Result<(PathBuf, PathBuf), String> {
    let project = project.unwrap_or(".");
    let project_path = Path::new(project).to_owned();

    let (project_dir, config_file) = if project_path.is_dir() {
        (project_path.clone(), project_path.join("rocat.yml"))
    } else if project_path.is_file() {
        (project_path.parent().unwrap().into(), project_path)
    } else {
        return Err(format!("Unable to parse project path: {}", project));
    };

    if config_file.exists() {
        return Ok((project_dir, config_file));
    }

    Err(format!(
        "Config file does not exist: {}",
        config_file.display()
    ))
}

fn get_current_branch() -> Result<String, String> {
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

    Ok(current_branch.to_owned())
}

fn get_state_file_path(project_path: &Path) -> PathBuf {
    project_path.join(".rocat-state.yml")
}

pub fn get_hash(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    format!("{:x}", digest)
}

pub fn get_file_hash(file_path: &Path) -> Result<String, String> {
    let buffer = fs::read(file_path).map_err(|e| {
        format!(
            "Failed to read file {} for hashing: {}",
            file_path.display(),
            e
        )
    })?;
    Ok(get_hash(&buffer))
}

pub fn get_previous_graph(
    project_path: &Path,
    config: &Config,
    deployment_config: &DeploymentConfig,
) -> Result<ResourceGraph, String> {
    let state_file_path = get_state_file_path(project_path);

    if !state_file_path.exists() {
        let mut resources: Vec<Resource> = Vec::new();

        let mut experience = Resource::new(resource_types::EXPERIENCE, SINGLETON_RESOURCE_ID)
            .add_output::<AssetId>("assetId", &deployment_config.experience_id.clone())?
            .clone();
        let experience_asset_id_ref = experience.get_input_ref("assetId");
        if let Some(_) = &config.templates.experience {
            experience.add_value_stub_input("configuration");
        }
        resources.push(experience.clone());

        for (name, id) in deployment_config.place_ids.iter() {
            let place_file = config
                .place_files
                .get(name)
                .ok_or(format!("No place file configured for place {}", name))?;
            let place_file_resource = Resource::new(resource_types::PLACE_FILE, name)
                .add_output("assetId", &id)?
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .add_value_input("filePath", &place_file)?
                .add_value_stub_input("fileHash")
                .add_value_stub_input("version")
                .add_value_stub_input("deployMode")
                .clone();
            let place_file_asset_id_ref = place_file_resource.get_input_ref("assetId");
            resources.push(place_file_resource);
            if let Some(_) = config.templates.places.get(name) {
                resources.push(
                    Resource::new(resource_types::PLACE_CONFIGURATION, name)
                        .add_ref_input("experienceId", &experience_asset_id_ref)
                        .add_ref_input("assetId", &place_file_asset_id_ref)
                        .add_value_stub_input("configuration")
                        .clone(),
                );
            }
        }

        return Ok(ResourceGraph::new(&resources));
    }

    let data = fs::read_to_string(&state_file_path).map_err(|e| {
        format!(
            "Unable to read state file: {}\n\t{}",
            state_file_path.display(),
            e
        )
    })?;

    let resources = serde_yaml::from_str::<Vec<Resource>>(&data).map_err(|e| {
        format!(
            "Unable to parse state file {}\n\t{}",
            state_file_path.display(),
            e
        )
    })?;

    Ok(ResourceGraph::new(&resources))
}

pub fn get_desired_graph(
    project_path: &Path,
    config: &Config,
    deployment_config: &DeploymentConfig,
) -> Result<ResourceGraph, String> {
    let mut resources: Vec<Resource> = Vec::new();

    let mut experience = Resource::new(resource_types::EXPERIENCE, SINGLETON_RESOURCE_ID)
        .add_output::<AssetId>("assetId", &deployment_config.experience_id.clone())?
        .clone();
    let experience_asset_id_ref = experience.get_input_ref("assetId");
    if let Some(experience_configuration) = &config.templates.experience {
        experience.add_value_input::<ExperienceConfigurationModel>(
            "configuration",
            &experience_configuration.into(),
        )?;
        resources.push(
            Resource::new(resource_types::EXPERIENCE_ACTIVATION, SINGLETON_RESOURCE_ID)
                .add_value_input(
                    "isActive",
                    &match experience_configuration.playability {
                        Some(PlayabilityConfig::Private) => false,
                        _ => true,
                    },
                )?
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .clone(),
        );
    }
    resources.push(experience.clone());

    for (name, id) in deployment_config.place_ids.iter() {
        let place_file = config
            .place_files
            .get(name)
            .ok_or(format!("No place file configured for place {}", name))?;
        let place_file_resource = Resource::new(resource_types::PLACE_FILE, name)
            .add_output("assetId", &id)?
            .add_ref_input("experienceId", &experience_asset_id_ref)
            .add_value_input("filePath", &place_file)?
            .add_value_input(
                "fileHash",
                &get_file_hash(project_path.join(place_file).as_path())?,
            )?
            .add_value_input("deployMode", &deployment_config.deploy_mode)?
            .clone();
        let place_file_asset_id_ref = place_file_resource.get_input_ref("assetId");
        resources.push(place_file_resource);
        if let Some(place_configuration) = config.templates.places.get(name) {
            resources.push(
                Resource::new(resource_types::PLACE_CONFIGURATION, name)
                    .add_ref_input("experienceId", &experience_asset_id_ref)
                    .add_ref_input("assetId", &place_file_asset_id_ref)
                    .add_value_input::<PlaceConfigurationModel>(
                        "configuration",
                        &place_configuration.clone().into(),
                    )?
                    .clone(),
            );
        }
    }

    if let Some(experience_configuration) = &config.templates.experience {
        if let Some(file_path) = &experience_configuration.icon {
            resources.push(
                Resource::new(resource_types::EXPERIENCE_ICON, file_path)
                    .add_ref_input("experienceId", &experience_asset_id_ref)
                    .add_value_input("filePath", file_path)?
                    .add_value_input(
                        "fileHash",
                        &get_file_hash(project_path.join(file_path).as_path())?,
                    )?
                    .clone(),
            );
        }
        if let Some(thumbnails) = &experience_configuration.thumbnails {
            let mut thumbnail_asset_id_refs: Vec<InputRef> = Vec::new();
            for file_path in thumbnails {
                let thumbnail_resource =
                    Resource::new(resource_types::EXPERIENCE_THUMBNAIL, file_path)
                        .add_ref_input("experienceId", &experience_asset_id_ref)
                        .add_value_input("filePath", file_path)?
                        .add_value_input(
                            "fileHash",
                            &get_file_hash(project_path.join(file_path).as_path())?,
                        )?
                        .clone();
                thumbnail_asset_id_refs.push(thumbnail_resource.get_input_ref("assetId"));
                resources.push(thumbnail_resource);
            }
            resources.push(
                Resource::new(
                    resource_types::EXPERIENCE_THUMBNAIL_ORDER,
                    SINGLETON_RESOURCE_ID,
                )
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .add_ref_input_list("assetIds", &thumbnail_asset_id_refs)
                .clone(),
            );
        }
    }

    Ok(ResourceGraph::new(&resources))
}

fn save_state(project_path: &Path, resources: &Vec<Resource>) -> Result<(), String> {
    let state_file_path = get_state_file_path(project_path);

    let data = serde_yaml::to_vec(&resources)
        .map_err(|e| format!("Unable to serialize state\n\t{}", e))?;

    fs::write(&state_file_path, data).map_err(|e| {
        format!(
            "Unable to write state file: {}\n\t{}",
            state_file_path.display(),
            e
        )
    })?;

    Ok(())
}

pub fn run(project: Option<&str>) -> Result<(), String> {
    let (project_path, config_file) = parse_project(project)?;

    let config = load_config_file(&config_file)?;

    let current_branch = get_current_branch()?;

    let deployment_config = config
        .deployments
        .iter()
        .find(|deployment| match_branch(&current_branch, &deployment.branches));

    let deployment_config = match deployment_config {
        Some(v) => v,
        None => {
            println!("No deployment configuration found for branch; no deployment necessary.");
            return Ok(());
        }
    };

    let mut resource_manager =
        ResourceManager::new(Box::new(RobloxResourceManager::new(&project_path)));
    let previous_graph =
        get_previous_graph(project_path.clone().as_path(), &config, &deployment_config)?;
    let mut next_graph = get_desired_graph(project_path.as_path(), &config, &deployment_config)?;
    next_graph.resolve(&mut resource_manager, &previous_graph)?;
    let resources = next_graph.get_resource_list();
    save_state(&project_path, &resources)?;

    Ok(())
}
