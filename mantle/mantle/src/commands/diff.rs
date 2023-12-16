use std::{fs, str};

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

pub async fn run(
    project: Option<&str>,
    environment: Option<&str>,
    output: Option<&str>,
    format: Option<&str>,
) -> i32 {
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
            let outputs_string = format.map(|format| match format {
                "json" => serde_json::to_string_pretty(&diff)
                    .map(|x| x + "\n")
                    .map_err(|e| e.to_string()),
                "yaml" => serde_yaml::to_string(&diff).map_err(|e| e.to_string()),
                _ => Err(format!("Unknown format: {}", format)),
            });

            print_diff(diff);
            logger::end_action("Succeeded");

            if let Some(outputs_string) = outputs_string {
                if let Ok(outputs_string) = outputs_string {
                    if let Some(output) = output {
                        if let Err(e) = fs::write(output, outputs_string).map_err(|e| {
                            format!("Unable to write outputs file: {}\n\t{}", output, e)
                        }) {
                            logger::log(Paint::red(e));
                            return 1;
                        }
                    } else {
                        print!("{}", outputs_string);
                    }
                } else {
                    logger::log(Paint::red("Failed to serialize outputs"));
                    return 1;
                }
            }

            0
        }
        Err(e) => {
            logger::end_action(Paint::red(e));
            1
        }
    }
}
