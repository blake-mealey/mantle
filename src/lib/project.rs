use std::{path::PathBuf, process::Command, str};

use yansi::Paint;

use super::{
    config::{
        Config, EnvironmentConfig, ExperienceTargetConfig, ExperienceTargetConfigurationConfig,
        OwnerConfig, PaymentsConfig, PlaceTargetConfigurationConfig, PlayabilityTargetConfig,
        StateConfig, TargetAccessConfig, TargetConfig, TargetNamePrefixConfig,
    },
    logger,
    resource_graph::ResourceGraph,
    roblox_api::{CreatorType, DEFAULT_PLACE_NAME},
    roblox_resource_manager::{RobloxInputs, RobloxOutputs, RobloxResource},
    state::{get_previous_state, ResourceStateVLatest},
};

fn run_command(dir: PathBuf, command: &str) -> std::io::Result<std::process::Output> {
    if cfg!(target_os = "windows") {
        return Command::new("cmd")
            .current_dir(dir)
            .arg("/C")
            .arg(command)
            .output();
    } else {
        return Command::new("sh")
            .current_dir(dir)
            .arg("-c")
            .arg(command)
            .output();
    }
}

fn get_current_branch(project_path: PathBuf) -> Result<String, String> {
    let output = run_command(project_path, "git symbolic-ref --short HEAD");
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

fn match_branch(branch: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        let glob_pattern = glob::Pattern::new(pattern);
        if glob_pattern.is_ok() && glob_pattern.unwrap().matches(branch) {
            return true;
        }
    }
    false
}

fn override_yaml(a: &mut serde_yaml::Value, b: serde_yaml::Value) {
    match (a, b) {
        (a @ &mut serde_yaml::Value::Mapping(_), serde_yaml::Value::Mapping(b)) => {
            let a = a.as_mapping_mut().unwrap();
            for (k, v) in b {
                if !v.is_null() {
                    if a.contains_key(&k) {
                        override_yaml(&mut a[&k], v);
                    } else {
                        a.insert(k.to_owned(), v.to_owned());
                    }
                }
            }
        }
        (a, b) => *a = b,
    }
}

fn get_target_config(
    environment: EnvironmentConfig,
    target: TargetConfig,
) -> Result<TargetConfig, String> {
    let target = match target {
        TargetConfig::Experience(mut experience) => {
            // Apply the name prefix to all places in the experience
            if let Some(target_name_prefix) = environment.target_name_prefix {
                let name_prefix = match target_name_prefix {
                    TargetNamePrefixConfig::Custom(prefix) => prefix,
                    TargetNamePrefixConfig::EnvironmentLabel => {
                        format!("[{}] ", environment.label.to_uppercase())
                    }
                };
                if let Some(places) = &mut experience.places {
                    for (_, place) in places.iter_mut() {
                        if let Some(config) = &mut place.configuration {
                            let name = match config.name.clone() {
                                Some(name) => name,
                                None => DEFAULT_PLACE_NAME.to_owned(),
                            };
                            config.name = Some(format!("{}{}", name_prefix, name));
                        } else {
                            place.configuration = Some(PlaceTargetConfigurationConfig {
                                name: Some(format!("{}{}", name_prefix, DEFAULT_PLACE_NAME)),
                                ..Default::default()
                            })
                        }
                    }
                }
            }

            // Apply the access to all places in the experience
            if let Some(target_access) = environment.target_access {
                let playability = Some(match target_access {
                    TargetAccessConfig::Public => PlayabilityTargetConfig::Public,
                    TargetAccessConfig::Private => PlayabilityTargetConfig::Private,
                    TargetAccessConfig::Friends => PlayabilityTargetConfig::Friends,
                });
                if let Some(config) = &mut experience.configuration {
                    config.playability = playability
                } else {
                    experience.configuration = Some(ExperienceTargetConfigurationConfig {
                        playability,
                        ..Default::default()
                    });
                }
            }

            // Apply overrides last (they are the final trump)
            if let Some(overrides) = environment.overrides {
                let overrides = serde_yaml::to_value(overrides).unwrap();
                let mut as_value = serde_yaml::to_value(experience)
                    .map_err(|e| format!("Failed to serialize target: {}", e))?;
                override_yaml(&mut as_value, overrides);
                experience = serde_yaml::from_value::<ExperienceTargetConfig>(as_value)
                    .map_err(|e| format!("Failed to deserialize target: {}", e))?;
            };

            TargetConfig::Experience(experience)
        }
    };
    Ok(target)
}

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
    project_path: PathBuf,
    config: Config,
    environment: Option<&str>,
) -> Result<Option<Project>, String> {
    let current_branch = get_current_branch(project_path.clone())?;

    let environment_config = match environment {
        Some(label) => {
            if let Some(result) = config.environments.iter().find(|d| d.label == label) {
                logger::log(format!(
                    "Selected provided environment configuration {}",
                    Paint::cyan(label)
                ));
                result
            } else {
                return Err(format!(
                    "No environment configuration found with name {}",
                    label
                ));
            }
        }
        None => {
            if let Some(result) = config
                .environments
                .iter()
                .find(|environment| match_branch(&current_branch, &environment.branches))
            {
                logger::log(format!(
                    "Selected environment configuration {} because the current branch {} matched one of [{}]",
                    Paint::cyan(result.label.clone()),
                    Paint::cyan(current_branch),
                    result.branches.iter().map(|b|Paint::cyan(b).to_string()).collect::<Vec<String>>().join(", ")
                ));
                result
            } else {
                logger::log(format!(
                    "No environment configuration found for the current branch {}",
                    Paint::cyan(current_branch)
                ));
                return Ok(None);
            }
        }
    };

    let target_config = get_target_config(environment_config.clone(), config.target.clone())?;

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
    let state = get_previous_state(project_path.as_path(), &config, environment_config).await?;

    // Get our resource graphs
    let previous_graph =
        ResourceGraph::new(state.environments.get(&environment_config.label).unwrap());

    Ok(Some(Project {
        current_graph: previous_graph,
        state,
        environment_config: environment_config.clone(),
        target_config,
        payment_source,
        state_config: config.state.clone(),
        owner_config: config.owner,
    }))
}
