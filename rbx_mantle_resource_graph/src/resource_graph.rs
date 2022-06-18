use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

use serde::Serialize;

use crate::resource::{Resource, ResourceId};

pub struct ResourceGraph<TResource, TInputs, TOutputs>
where
    TResource: Resource<TInputs, TOutputs>,
    TInputs: Clone,
    TOutputs: Clone,
{
    phantom_inputs: std::marker::PhantomData<TInputs>,
    phantom_outputs: std::marker::PhantomData<TOutputs>,
    pub resources: HashMap<ResourceId, TResource>,
}

impl<TResource, TInputs, TOutputs> ResourceGraph<TResource, TInputs, TOutputs>
where
    TResource: Resource<TInputs, TOutputs>,
    TInputs: Clone,
    TOutputs: Clone,
    TOutputs: Serialize,
{
    pub fn new(resources: &[TResource]) -> Self {
        Self {
            resources: resources
                .iter()
                .map(|resource| (resource.get_id(), resource.clone()))
                .collect(),
            phantom_inputs: PhantomData,
            phantom_outputs: PhantomData,
        }
    }

    pub fn get_resource(&self, resource_id: &str) -> Option<&TResource> {
        self.resources.get(resource_id)
    }

    pub fn insert_resource(&mut self, resource: TResource) {
        self.resources.insert(resource.get_id(), resource);
    }

    pub fn get_outputs(&self, resource_id: &str) -> Option<TOutputs> {
        self.resources
            .get(resource_id)
            .and_then(|resource| resource.get_outputs())
    }

    pub fn get_dependency_graph(&self) -> BTreeMap<ResourceId, Vec<ResourceId>> {
        self.resources
            .iter()
            .map(|(id, resource)| (id.clone(), resource.get_dependencies()))
            .collect()
    }

    pub fn get_topological_order(&self) -> Result<Vec<ResourceId>, String> {
        let mut dependency_graph = self.get_dependency_graph();

        let mut start_nodes: Vec<ResourceId> = dependency_graph
            .iter()
            .filter_map(|(node, deps)| {
                if deps.is_empty() {
                    Some(node.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut ordered: Vec<ResourceId> = Vec::new();
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

    pub fn get_resource_list(&self) -> Vec<TResource> {
        self.get_topological_order()
            .unwrap()
            .iter()
            .map(|id| self.resources.get(id).unwrap().clone())
            .collect()
    }

    pub fn get_dependency_outputs(&self, resource: &TResource) -> Option<Vec<TOutputs>> {
        let mut dependency_outputs: Vec<TOutputs> = Vec::new();
        for dependency in resource.get_dependencies() {
            let resource = self.resources.get(&dependency);
            if let Some(resource) = resource {
                if let Some(outputs) = resource.get_outputs() {
                    dependency_outputs.push(outputs);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        Some(dependency_outputs)
    }

    pub fn get_dependency_outputs_hash(&self, dependency_outputs: Vec<TOutputs>) -> String {
        // TODO: Should we separate hashes from displays?
        let hash = serde_yaml::to_string(&dependency_outputs)
            .map_err(|e| format!("Failed to compute dependency outputs hash\n\t{}", e))
            .unwrap();
        if hash.is_empty() {
            ""
        } else {
            // We remove first 4 characters to remove "---\n", and we trim the end to remove "\n"
            hash[4..].trim_end()
        }
        .to_owned()
    }
}
