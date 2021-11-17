use yansi::Paint;

use crate::{
    logger,
    project::{load_project, Project},
    resource_manager::AssetId,
    roblox_api::RobloxApi,
    roblox_auth::RobloxAuth,
    state::{import_graph, save_state},
};

pub async fn run(project: Option<&str>, deployment: Option<&str>, experience_id: &str) -> i32 {
    logger::start_action("Loading project:");
    let Project {
        project_path,
        previous_graph,
        mut state,
        deployment_config,
        state_config,
        ..
    } = match load_project(project, deployment).await {
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

    if !previous_graph.get_resource_list().is_empty() {
        logger::end_action("Deployment already exists: no need to import.");
        return 0;
    }

    logger::end_action("Succeeded");

    let experience_id = match experience_id.parse::<AssetId>() {
        Ok(v) => v,
        Err(e) => {
            logger::log(Paint::red(format!(
                "Experience ID {} is invalid: {}",
                experience_id, e
            )));
            return 1;
        }
    };

    logger::start_action("Import experience:");
    let mut roblox_api = RobloxApi::new(RobloxAuth::new());
    let imported_graph = match import_graph(&mut roblox_api, experience_id) {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(format!("Failed: {}", e)));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    logger::start_action("Saving state:");
    state.deployments.insert(
        deployment_config.name.clone(),
        imported_graph.get_resource_list(),
    );
    match save_state(&project_path, &state_config, &state).await {
        Ok(_) => {}
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    0
}
