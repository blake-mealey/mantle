use std::str;

use yansi::Paint;

use crate::{
    logger,
    project::{load_project, Project},
    roblox_resource_manager::RobloxResourceManager,
    resource_graph::{EvaluateResults, ResourceGraph},
    state::save_state,
};

pub async fn run(project: Option<&str>, environment: Option<&str>) -> i32 {
    logger::start_action("Loading project:");
    let Project {
        project_path,
        previous_graph,
        mut state,
        environment_config,
        payment_source,
        state_config,
        ..
    } = match load_project(project, environment).await {
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

    let mut resource_manager = RobloxResourceManager::new(&project_path, payment_source);

    logger::start_action("Destroying resources:");
    let mut next_graph = ResourceGraph::new(&Vec::new());
    let results = next_graph.evaluate(&previous_graph, &mut resource_manager, false);
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
    state.environments.remove(&environment_config.name);
    match save_state(&project_path, &state_config, &state).await {
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
