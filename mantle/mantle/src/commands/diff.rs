use std::str;

use difference::Changeset;
use yansi::Paint;

use rbx_mantle::{
    config::load_project_config,
    project::{load_project, Project},
    resource_graph::ResourceGraphDiff,
    state::get_desired_graph,
};

fn get_changeset(previous_hash: &str, new_hash: &str) -> Changeset {
    Changeset::new(previous_hash, new_hash, "\n")
}

fn print_diff(diff: ResourceGraphDiff) {
    for (resource_id, r) in diff.removals.into_iter() {
        logger::start_action(format!("{} Removed {}:", Paint::red("-"), resource_id));
        logger::log("Inputs:");
        logger::log_changeset(get_changeset(&r.previous_inputs_hash, ""));
        logger::end_action_without_message();
    }

    for (resource_id, r) in diff.additions.into_iter() {
        logger::start_action(format!("{} Added {}:", Paint::green("+"), resource_id));
        logger::log("Inputs:");
        logger::log_changeset(get_changeset("", &r.current_inputs_hash));
        logger::end_action_without_message();
    }

    for (resource_id, r) in diff.changes.into_iter() {
        logger::start_action(format!("{} Changed {}:", Paint::yellow("~"), resource_id));
        logger::log("Inputs:");
        logger::log_changeset(get_changeset(
            &r.previous_inputs_hash,
            &r.current_inputs_hash,
        ));
        logger::end_action_without_message();
    }

    for (resource_id, r) in diff.dependency_changes.into_iter() {
        logger::start_action(format!(
            "{} Dependency Changed {}:",
            Paint::new("○").dimmed(),
            resource_id
        ));
        logger::log("Changed dependencies:");
        for dependency_id in r.changed_dependencies.into_iter() {
            logger::log(format!(
                " {} {}",
                Paint::new("-").dimmed(),
                Paint::yellow(dependency_id)
            ))
        }
        logger::end_action_without_message();
    }
}

pub async fn run(project: Option<&str>, environment: Option<&str>) -> i32 {
    logger::start_action("Loading project:");
    let (project_path, config) = match load_project_config(project) {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    let Project {
        current_graph,
        target_config,
        owner_config,
        ..
    } = match load_project(project_path.clone(), config, environment).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            logger::end_action("No diff available");
            return 0;
        }
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    let mut next_graph =
        match get_desired_graph(project_path.as_path(), &target_config, &owner_config) {
            Ok(v) => v,
            Err(e) => {
                logger::end_action(Paint::red(e));
                return 1;
            }
        };
    logger::end_action("Succeeded");

    logger::start_action("Diffing resource graphs:");

    let diff = next_graph.diff(&current_graph);

    match diff {
        Ok(diff) => {
            print_diff(diff);
            logger::end_action("Succeeded");
            return 0;
        }
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    }
}
