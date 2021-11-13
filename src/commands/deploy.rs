use std::str;

use serde::de;
use yansi::Paint;

use crate::{
    config::DeploymentConfig,
    logger,
    project::{load_project, Project},
    resource_manager::{resource_types, RobloxResourceManager},
    resources::{EvaluateResults, InputRef, ResourceGraph},
    state::save_state,
    util::run_command,
};

fn get_output<T>(graph: &ResourceGraph, input_ref: &InputRef) -> Option<T>
where
    T: de::DeserializeOwned,
{
    graph
        .get_resource_from_input_ref(input_ref)
        .map(|r| {
            r.get_output_from_input_ref(input_ref)
                .ok()
                .map(|v| serde_yaml::from_value::<T>(v).ok())
                .flatten()
        })
        .flatten()
}

fn tag_commit(
    deployment_config: &DeploymentConfig,
    next_graph: &ResourceGraph,
    previous_graph: &ResourceGraph,
) -> Result<u32, String> {
    let mut tag_count: u32 = 0;
    for name in deployment_config.place_ids.keys() {
        let input_ref = (
            resource_types::PLACE_FILE.to_owned(),
            name.to_owned(),
            "version".to_owned(),
        );
        let previous_version_output = get_output::<u32>(previous_graph, &input_ref);
        let next_version_output = get_output::<u32>(next_graph, &input_ref);

        let tag_version = match (previous_version_output, next_version_output) {
            (None, Some(version)) => Some(version),
            (Some(previous), Some(next)) if next != previous => Some(next),
            _ => None,
        };

        if let Some(version) = tag_version {
            logger::log(format!(
                "Place {} was updated to version {}",
                Paint::cyan(name),
                Paint::cyan(version)
            ));
            let tag = format!("{}-v{}", name, version);
            logger::log(format!("Tagging commit with {}", Paint::cyan(tag.clone())));

            tag_count += 1;
            run_command(&format!("git tag {}", tag))
                .map_err(|e| format!("Unable to tag the current commit\n\t{}", e))?;
        }
    }

    if tag_count > 0 {
        run_command("git push --tags")
            .map_err(|e| format!("Unable to push tags to remote\n\t{}", e))?;
    }

    Ok(tag_count)
}

pub async fn run(project: Option<&str>, deployment: Option<&str>) -> i32 {
    logger::start_action("Loading project:");
    let Project {
        project_path,
        mut next_graph,
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
    logger::end_action("Succeeded");

    let mut resource_manager = RobloxResourceManager::new(&project_path);

    logger::start_action("Deploying resources:");
    let results = next_graph.evaluate(&previous_graph, &mut resource_manager);
    match &results {
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
        }
        Err(e) => {
            logger::end_action(Paint::red(e));
        }
    };

    if deployment_config.tag_commit && matches!(results, Ok(_)) {
        logger::start_action("Tagging commit:");
        match tag_commit(&deployment_config, &next_graph, &previous_graph) {
            Ok(0) => logger::end_action("No tagging required"),
            Ok(tag_count) => {
                logger::end_action(format!("Succeeded in pushing {} tag(s)", tag_count))
            }
            Err(e) => logger::end_action(Paint::red(e)),
        };
    }

    logger::start_action("Saving state:");
    state.deployments.insert(
        deployment_config.name.clone(),
        next_graph.get_resource_list(),
    );
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
