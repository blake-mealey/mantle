use std::{
    path::{Path, PathBuf},
    process::Command,
    str,
};

use yansi::Paint;

use crate::{
    config::load_config_file,
    logger::logger,
    resource_manager::RobloxResourceManager,
    resources::{EvaluateResults, ResourceGraph, ResourceManager},
    state::{get_desired_graph, get_previous_state, save_state},
};

fn run_command(command: &str) -> std::io::Result<std::process::Output> {
    if cfg!(target_os = "windows") {
        return Command::new("cmd").arg("/C").arg(command).output();
    } else {
        return Command::new("sh").arg("-c").arg(command).output();
    }
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

pub async fn run(project: Option<&str>) -> i32 {
    let (project_path, config_file) = match parse_project(project) {
        Ok(v) => v,
        Err(e) => {
            logger::log_error(e);
            return 1;
        }
    };

    let config = match load_config_file(&config_file) {
        Ok(v) => v,
        Err(e) => {
            logger::log_error(e);
            return 1;
        }
    };

    let current_branch = match get_current_branch() {
        Ok(v) => v,
        Err(e) => {
            logger::log_error(e);
            return 1;
        }
    };

    let deployment_config = config
        .deployments
        .iter()
        .find(|deployment| match_branch(&current_branch, &deployment.branches));

    let deployment_config = match deployment_config {
        Some(v) => v,
        None => {
            logger::log("No deployment configuration found for branch; no deployment necessary.");
            return 0;
        }
    };

    // Get our resource manager
    let mut resource_manager =
        ResourceManager::new(Box::new(RobloxResourceManager::new(&project_path)));

    // Get previous state
    let mut state =
        match get_previous_state(project_path.as_path(), &config, deployment_config).await {
            Ok(v) => v,
            Err(e) => {
                logger::log_error(e);
                return 1;
            }
        };

    // Get our resource graphs
    let previous_graph =
        ResourceGraph::new(state.deployments.get(&deployment_config.name).unwrap());
    let mut next_graph = match get_desired_graph(project_path.as_path(), &config, deployment_config)
    {
        Ok(v) => v,
        Err(e) => {
            logger::log_error(e);
            return 1;
        }
    };

    // Evaluate the resource graph
    logger::start_action("Evaluating resource graph:");
    let exit_code = match next_graph.evaluate(&previous_graph, &mut resource_manager) {
        Ok(results) => {
            match results {
                EvaluateResults {
                    created_count: 0,
                    updated_count: 0,
                    deleted_count: 0,
                    ..
                } => logger::end_action("Succeeded with no changes required"),
                EvaluateResults {
                    created_count,
                    updated_count,
                    deleted_count,
                    noop_count,
                } => logger::end_action(format!(
                    "Succeeded with {} create(s), {} update(s), {} delete(s), {} noop(s)",
                    created_count, updated_count, deleted_count, noop_count
                )),
            };
            0
        }
        Err(e) => {
            logger::end_action(Paint::red(e));
            1
        }
    };

    // Save the results to the state file
    state.deployments.insert(
        deployment_config.name.clone(),
        next_graph.get_resource_list(),
    );
    match save_state(&project_path, &config.state, &state).await {
        Ok(_) => {}
        Err(e) => {
            logger::log_error(e);
            return 1;
        }
    };

    // If there were errors, return them
    exit_code
}
