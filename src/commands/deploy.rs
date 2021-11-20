use std::str;

use yansi::Paint;

use crate::{
    config::TargetConfig,
    logger,
    project::{load_project, Project},
    safe_resource_manager::{RobloxInputs, RobloxOutputs, RobloxResource, RobloxResourceManager},
    safe_resources::{EvaluateResults, ResourceGraph},
    state::save_state,
    util::run_command,
};

fn tag_commit(
    target_config: &TargetConfig,
    next_graph: &ResourceGraph<RobloxResource, RobloxInputs, RobloxOutputs>,
    previous_graph: &ResourceGraph<RobloxResource, RobloxInputs, RobloxOutputs>,
) -> Result<u32, String> {
    let mut tag_count: u32 = 0;

    #[allow(irrefutable_let_patterns)]
    if let TargetConfig::Experience(target_config) = target_config {
        for name in target_config.places.as_ref().unwrap().keys() {
            let input_id = format!("placeFile_{}", name);

            let previous_outputs = previous_graph.get_outputs(&input_id);
            let next_outputs = next_graph.get_outputs(&input_id);

            let tag_version = match (previous_outputs, next_outputs) {
                (None, Some(RobloxOutputs::PlaceFile(next))) => Some(next.version),
                (
                    Some(RobloxOutputs::PlaceFile(previous)),
                    Some(RobloxOutputs::PlaceFile(next)),
                ) if next.version != previous.version => Some(next.version),
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
    }

    if tag_count > 0 {
        run_command("git push --tags")
            .map_err(|e| format!("Unable to push tags to remote\n\t{}", e))?;
    }
    Ok(tag_count)
}

pub async fn run(project: Option<&str>, environment: Option<&str>, allow_purchases: bool) -> i32 {
    logger::start_action("Loading project:");
    let Project {
        project_path,
        mut next_graph,
        previous_graph,
        mut state,
        environment_config,
        target_config,
        payment_source,
        state_config,
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

    logger::start_action("Deploying resources:");
    let results = next_graph.evaluate(&previous_graph, &mut resource_manager, allow_purchases);
    match &results {
        Ok(results) => {
            match results {
                EvaluateResults {
                    created_count: 0,
                    updated_count: 0,
                    deleted_count: 0,
                    skipped_count: 0,
                    ..
                } => logger::end_action("No changes required"),
                EvaluateResults {
                    created_count,
                    updated_count,
                    deleted_count,
                    noop_count,
                    skipped_count,
                } => logger::end_action(format!(
                    "Succeeded with {} create(s), {} update(s), {} delete(s), {} noop(s), {} skip(s)",
                    created_count, updated_count, deleted_count, noop_count, skipped_count
                )),
            };
        }
        Err(e) => {
            logger::end_action(Paint::red(e));
        }
    };

    if environment_config.tag_commit && matches!(results, Ok(_)) {
        logger::start_action("Tagging commit:");
        match tag_commit(&target_config, &next_graph, &previous_graph) {
            Ok(0) => logger::end_action("No tagging required"),
            Ok(tag_count) => {
                logger::end_action(format!("Succeeded in pushing {} tag(s)", tag_count))
            }
            Err(e) => logger::end_action(Paint::red(e)),
        };
    }

    logger::start_action("Saving state:");
    state.environments.insert(
        environment_config.name.clone(),
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
