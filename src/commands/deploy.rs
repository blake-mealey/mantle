use std::str;

use yansi::Paint;

use crate::{
    logger,
    project::{load_project, Project},
    resource_manager::RobloxResourceManager,
    resources::{EvaluateResults, ResourceManager},
    state::save_state,
};

pub async fn run(project: Option<&str>) -> i32 {
    logger::start_action("Loading project:");
    let Project {
        project_path,
        config,
        deployment_config,
        mut next_graph,
        previous_graph,
        mut state,
    } = match load_project(project).await {
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

    let mut resource_manager =
        ResourceManager::new(Box::new(RobloxResourceManager::new(&project_path)));

    logger::start_action("Deploying resources:");
    let exit_code = match next_graph.evaluate(&previous_graph, &mut resource_manager) {
        Ok(results) => {
            match results {
                EvaluateResults {
                    created_count: 0,
                    updated_count: 0,
                    deleted_count: 0,
                    ..
                } => logger::end_action("No changes required"),
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

    logger::start_action("Saving state:");
    state.deployments.insert(
        deployment_config.name.clone(),
        next_graph.get_resource_list(),
    );
    match save_state(&project_path, &config.state, &state).await {
        Ok(_) => {}
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    exit_code
}
