use std::{collections::BTreeMap, fs};

use rbx_mantle_resource_graph::resource::Resource;
use yansi::Paint;

use rbx_mantle::{
    config::load_project_config,
    project::{load_project, Project},
};

pub async fn run(
    project: Option<&str>,
    environment: Option<&str>,
    output: Option<&str>,
    format: &str,
) -> i32 {
    logger::start_action("Load outputs:");
    let (project_path, config) = match load_project_config(project) {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    let Project { current_graph, .. } =
        match load_project(project_path.clone(), config, environment).await {
            Ok(Some(v)) => v,
            Ok(None) => {
                logger::end_action("No outputs available");
                return 0;
            }
            Err(e) => {
                logger::end_action(Paint::red(e));
                return 1;
            }
        };

    let resources = current_graph.get_resource_list();
    let outputs_map = resources
        .iter()
        .map(|r| (r.get_id(), r.get_outputs()))
        .collect::<BTreeMap<_, _>>();

    let outputs_string = match match format {
        "json" => serde_json::to_string_pretty(&outputs_map).map_err(|e| e.to_string()),
        "yaml" => serde_yaml::to_string(&outputs_map).map_err(|e| e.to_string()),
        _ => Err(format!("Unknown format: {}", format)),
    } {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(format!("Failed to serialize outputs: {}", e)));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    if let Some(output) = output {
        if let Err(e) = fs::write(output, outputs_string)
            .map_err(|e| format!("Unable to write outputs file: {}\n\t{}", output, e))
        {
            logger::log(Paint::red(e));
            return 1;
        }
    } else {
        logger::log(outputs_string);
    }

    0
}
