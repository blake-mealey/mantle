use std::str;

use yansi::Paint;

use rbx_mantle::{
    config::load_project_config,
    project::{load_project, Project},
    resource_graph::{EvaluateResults, ResourceGraph},
    roblox_resource_manager::RobloxResourceManager,
    state::save_state,
};

pub async fn run(project: Option<&str>, environment: Option<&str>) -> i32 {
    logger::start_action("Loading project:");
    let config_file = match load_project_config(project) {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    let Project {
        current_graph,
        mut state,
        environment_config,
        payment_source,
        state_config,
        ..
    } = match load_project(&config_file, environment).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            logger::end_action("No deployment necessary");
            return 0;
        }
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    logger::start_action("Destroying resources:");
    let mut resource_manager =
        match RobloxResourceManager::new(&config_file.project_path, payment_source).await {
            Ok(v) => v,
            Err(e) => {
                logger::end_action(Paint::red(e));
                return 1;
            }
        };

    let mut next_graph = ResourceGraph::new(&Vec::new());
    let results = next_graph
        .evaluate(&current_graph, &mut resource_manager, false)
        .await;
    match &results {
        Ok(results) => {
            match results {
                EvaluateResults {
                    deleted_count: 0, ..
                } => logger::end_action("No changes required"),
                EvaluateResults { deleted_count, .. } => {
                    logger::end_action(format!("Succeeded with {} delete(s)", deleted_count))
                }
            };
        }
        Err(e) => {
            logger::end_action(Paint::red(e));
        }
    };

    logger::start_action("Saving state:");
    let resource_list = next_graph.get_resource_list();
    if resource_list.is_empty() {
        state.environments.remove(&environment_config.label);
    } else {
        state.environments.insert(
            environment_config.label.clone(),
            next_graph.get_resource_list(),
        );
    }
    match save_state(&config_file.project_path, &state_config, &state).await {
        Ok(_) => {}
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    match &results {
        Ok(_) => 0,
        Err(_) => 1,
    }
}
