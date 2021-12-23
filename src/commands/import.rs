use yansi::Paint;

use crate::lib::{
    logger,
    project::{load_project, Project},
    roblox_api::RobloxApi,
    roblox_auth::RobloxAuth,
    roblox_resource_manager::AssetId,
    state::{import_graph, save_state},
};

pub async fn run(project: Option<&str>, environment: Option<&str>, target_id: &str) -> i32 {
    logger::start_action("Loading project:");
    let Project {
        project_path,
        previous_graph,
        mut state,
        environment_config,
        state_config,
        ..
    } = match load_project(project, environment).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            logger::end_action("No import necessary");
            return 0;
        }
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };

    if !previous_graph.get_resource_list().is_empty() {
        logger::end_action("Environment state already exists: no need to import.");
        return 0;
    }

    logger::end_action("Succeeded");

    let target_id = match target_id.parse::<AssetId>() {
        Ok(v) => v,
        Err(e) => {
            logger::log(Paint::red(format!(
                "Experience ID {} is invalid: {}",
                target_id, e
            )));
            return 1;
        }
    };

    logger::start_action("Import target:");
    let roblox_auth = match RobloxAuth::new().await {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    let roblox_api = match RobloxApi::new(roblox_auth).await {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };

    let imported_graph = match import_graph(&roblox_api, target_id).await {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(format!("Failed: {}", e)));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    logger::start_action("Saving state:");
    state.environments.insert(
        environment_config.name.clone(),
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
