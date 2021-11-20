use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LegacyResource {
    pub resource_type: String,
    pub id: String,
    pub inputs: BTreeMap<String, Input>,
    pub outputs: Option<BTreeMap<String, OutputValue>>,
}

impl LegacyResource {
    pub fn get_ref(&self) -> ResourceRef {
        (self.resource_type.clone(), self.id.clone())
    }

    fn get_dependency_refs(&self) -> Vec<ResourceRef> {
        self.inputs
            .values()
            .filter_map(|input| match input {
                Input::Ref(ref input_ref) => Some(vec![input_ref]),
                Input::RefList(ref input_ref_list) => Some(input_ref_list.iter().collect()),
                _ => None,
            })
            .flatten()
            .map(resource_ref_from_input_ref)
            .collect()
    }
}

pub type InputRef = (String, String, String);
pub type InputValue = serde_yaml::Value;
pub type OutputValue = serde_yaml::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Input {
    Ref(InputRef),
    RefList(Vec<InputRef>),
    Value(InputValue),
}

pub type ResourceRef = (String, String);

fn resource_ref_from_input_ref(input_ref: &InputRef) -> ResourceRef {
    let (input_ref_type, input_ref_id, _) = input_ref;
    (input_ref_type.clone(), input_ref_id.clone())
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LegacyResourceGraph {
    pub resources: HashMap<ResourceRef, LegacyResource>,
}

impl LegacyResourceGraph {
    pub fn new(resources: &[LegacyResource]) -> Self {
        let mut graph = LegacyResourceGraph {
            resources: HashMap::new(),
        };
        for resource in resources {
            graph = graph.add_resource(resource);
        }
        graph
    }

    pub fn add_resource(mut self, resource: &LegacyResource) -> Self {
        self.resources.insert(resource.get_ref(), resource.clone());
        self
    }

    pub fn get_resource_from_ref(&self, resource_ref: &ResourceRef) -> Option<LegacyResource> {
        self.resources.get(resource_ref).cloned()
    }
    fn get_dependency_graph(&self) -> BTreeMap<ResourceRef, Vec<ResourceRef>> {
        let mut dependency_graph = BTreeMap::new();
        for resource in self.resources.values() {
            dependency_graph.insert(resource.get_ref(), resource.get_dependency_refs());
        }
        dependency_graph
    }

    pub fn get_topological_order(&self) -> Result<Vec<ResourceRef>, String> {
        let mut dependency_graph = self.get_dependency_graph();

        let mut start_nodes: Vec<ResourceRef> = dependency_graph
            .iter()
            .filter_map(|(node, deps)| {
                if deps.is_empty() {
                    Some(node.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut ordered: Vec<ResourceRef> = Vec::new();
        while let Some(start_node) = start_nodes.pop() {
            ordered.push(start_node.clone());
            for (node, deps) in dependency_graph.iter_mut() {
                if deps.contains(&start_node) {
                    deps.retain(|dep| dep != &start_node);
                    if deps.is_empty() {
                        start_nodes.push(node.clone());
                    }
                }
            }
        }

        let has_cycles = dependency_graph.iter().any(|(_, deps)| !deps.is_empty());
        match has_cycles {
            true => Err("Cannot evaluate resource graph because it has cycles".to_owned()),
            false => Ok(ordered),
        }
    }
}
