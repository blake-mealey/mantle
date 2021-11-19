use std::{
    path::{Path, PathBuf},
    str,
};

use yansi::Paint;

use crate::{
    config::{
        load_config_file, EnvironmentConfig, ExperienceTargetConfig, OwnerConfig, PaymentsConfig,
        StateConfig, TargetConfig,
    },
    logger,
    roblox_api::CreatorType,
    safe_resource_manager::{RobloxInputs, RobloxOutputs, RobloxResource},
    safe_resources::ResourceGraph,
    safe_state::{get_desired_graph, get_previous_state, ResourceStateVLatest},
    util::run_command,
};

fn parse_project(project: Option<&str>) -> Result<(PathBuf, PathBuf), String> {
    let project = project.unwrap_or(".");
    let project_path = Path::new(project).to_owned();

    let (project_dir, config_file) = if project_path.is_dir() {
        (project_path.clone(), project_path.join("mantle.yml"))
    } else if project_path.is_file() {
        (project_path.parent().unwrap().into(), project_path)
    } else {
        return Err(format!("Unable to load project path: {}", project));
    };

    if config_file.exists() {
        return Ok((project_dir, config_file));
    }

    Err(format!("Config file {} not found", config_file.display()))
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
    target: TargetConfig,
    overrides: serde_yaml::Value,
) -> Result<TargetConfig, String> {
    let target = match target {
        TargetConfig::Experience(experience) => {
            let mut as_value = serde_yaml::to_value(experience)
                .map_err(|e| format!("Failed to serialize target: {}", e))?;
            override_yaml(&mut as_value, overrides);
            TargetConfig::Experience(
                serde_yaml::from_value::<ExperienceTargetConfig>(as_value)
                    .map_err(|e| format!("Failed to deserialize target: {}", e))?,
            )
        }
    };
    Ok(target)
}

pub struct Project {
    pub project_path: PathBuf,
    pub next_graph: ResourceGraph<RobloxResource, RobloxInputs, RobloxOutputs>,
    pub previous_graph: ResourceGraph<RobloxResource, RobloxInputs, RobloxOutputs>,
    pub state: ResourceStateVLatest,
    pub environment_config: EnvironmentConfig,
    pub target_config: TargetConfig,
    pub payment_source: CreatorType,
    pub state_config: StateConfig,
}

pub async fn load_project(
    project: Option<&str>,
    environment: Option<&str>,
) -> Result<Option<Project>, String> {
    let (project_path, config_file) = parse_project(project)?;

    let config = load_config_file(&config_file)?;
    logger::log(format!(
        "Loaded config file {}",
        Paint::cyan(config_file.display())
    ));

    let current_branch = get_current_branch()?;

    let environment_config = match environment {
        Some(name) => {
            if let Some(result) = config.environments.iter().find(|d| d.name == name) {
                logger::log(format!(
                    "Selected provided environment configuration {}",
                    Paint::cyan(name)
                ));
                result
            } else {
                return Err(format!(
                    "No environment configuration found with name {}",
                    name
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
                    Paint::cyan(result.name.clone()),
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

    let target_config = match &environment_config.overrides {
        Some(overrides) => get_target_config(config.target.clone(), overrides.clone())?,
        None => config.target.clone(),
    };

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
        ResourceGraph::new(state.environments.get(&environment_config.name).unwrap());
    let next_graph = get_desired_graph(project_path.as_path(), &target_config, &config.owner)?;

    Ok(Some(Project {
        project_path,
        next_graph,
        previous_graph,
        state,
        environment_config: environment_config.clone(),
        target_config,
        payment_source,
        state_config: config.state.clone(),
    }))
}
