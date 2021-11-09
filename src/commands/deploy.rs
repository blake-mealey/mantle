use std::{
    path::{Path, PathBuf},
    process::Command,
    str,
};

use crate::{
    config::load_config_file,
    logger::logger,
    resource_manager::RobloxResourceManager,
    resources::{ResourceGraph, ResourceManager},
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

pub async fn run(project: Option<&str>) -> Result<(), String> {
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

    // Get our resource manager
    let mut resource_manager =
        ResourceManager::new(Box::new(RobloxResourceManager::new(&project_path)));

    // Get previous state
    let mut state = get_previous_state(project_path.as_path(), &config, deployment_config).await?;

    // Get our resource graphs
    let previous_graph =
        ResourceGraph::new(state.deployments.get(&deployment_config.name).unwrap());
    let mut next_graph = get_desired_graph(project_path.as_path(), &config, deployment_config)?;

    // Evaluate the resource graph
    let result = next_graph.evaluate(&previous_graph, &mut resource_manager);

    // Save the results to the state file
    state.deployments.insert(
        deployment_config.name.clone(),
        next_graph.get_resource_list(),
    );
    save_state(&project_path, &config.state, &state).await?;

    // If there were errors, return them
    result?;

    Ok(())
}
