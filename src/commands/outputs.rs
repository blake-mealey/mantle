use std::{collections::BTreeMap, fs};

use yansi::Paint;

use crate::{
    logger,
    project::{load_project, Project},
    resources::ResourceRef,
};

pub async fn run(project: Option<&str>, output: Option<&str>, format: &str) -> i32 {
    logger::start_action("Load outputs:");
    let Project { previous_graph, .. } = match load_project(project).await {
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

    let resources = previous_graph.get_resource_list();
    let outputs_list: Vec<(ResourceRef, &BTreeMap<String, serde_yaml::Value>)> = resources
        .iter()
        .filter_map(|r| match &r.outputs {
            None => None,
            Some(v) => Some((r.get_ref(), v)),
        })
        .collect();

    let mut outputs_map: BTreeMap<String, BTreeMap<String, BTreeMap<String, serde_yaml::Value>>> =
        BTreeMap::new();
    for ((resource_type, resource_id), outputs) in outputs_list {
        let type_map = outputs_map.entry(resource_type).or_insert(BTreeMap::new());
        type_map.insert(resource_id, outputs.clone());
    }

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
