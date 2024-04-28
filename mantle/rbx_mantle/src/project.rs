use rbx_api::models::CreatorType;

use crate::config::ConfigFile;

use super::{
    config::{EnvironmentConfig, OwnerConfig, PaymentsConfig, StateConfig, TargetConfig},
    resource_graph::ResourceGraph,
    roblox_resource_manager::{RobloxInputs, RobloxOutputs, RobloxResource},
    state::{get_previous_state, ResourceStateVLatest},
};

pub struct Project {
    pub current_graph: ResourceGraph<RobloxResource, RobloxInputs, RobloxOutputs>,
    pub state: ResourceStateVLatest,
    pub environment_config: EnvironmentConfig,
    pub target_config: TargetConfig,
    pub payment_source: CreatorType,
    pub state_config: StateConfig,
    pub owner_config: OwnerConfig,
}

pub async fn load_project(
    config_file: &ConfigFile,
    environment: Option<&str>,
) -> Result<Option<Project>, String> {
    let environment_config = config_file
        .environment_config(environment)
        .map_err(|e| e.to_string())?;

    if let Some(environment_config) = environment_config {
        let config = config_file
            .config(environment_config)
            .map_err(|e| e.to_string())?;

        let payment_source = match config.payments {
            PaymentsConfig::Owner => match config.owner {
                OwnerConfig::Personal => CreatorType::User,
                OwnerConfig::Group(_) => CreatorType::Group,
            },
            PaymentsConfig::Personal => CreatorType::User,
            PaymentsConfig::Group => match config.owner {
                OwnerConfig::Personal => {
                    return Err(
                        "Cannot specify `payments: group` when owner is not a group.".to_owned(),
                    )
                }
                OwnerConfig::Group(_) => CreatorType::Group,
            },
        };

        // Get previous state
        let state = get_previous_state(
            config_file.project_path.as_path(),
            &config_file.header,
            environment_config,
        )
        .await?;

        // Get our resource graphs
        let previous_graph =
            ResourceGraph::new(state.environments.get(&environment_config.label).unwrap());

        Ok(Some(Project {
            current_graph: previous_graph,
            state,
            environment_config: environment_config.clone(),
            target_config: config.target,
            payment_source,
            state_config: config_file.header.state.clone(),
            owner_config: config.owner,
        }))
    } else {
        Ok(None)
    }
}
