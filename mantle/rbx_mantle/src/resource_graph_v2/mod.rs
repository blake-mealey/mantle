pub mod evaluator;
pub mod evaluator_results;

use std::collections::BTreeMap;

use crate::resources_v2::{RbxResource, ResourceGroup};

#[derive(Debug)]
pub struct ResourceGraph {
    resources: BTreeMap<String, RbxResource>,
}

impl ResourceGraph {
    pub fn new(resources: Vec<RbxResource>) -> Self {
        Self {
            resources: resources
                .into_iter()
                .map(|resource| (resource.id().to_owned(), resource))
                .collect(),
        }
    }

    pub fn default() -> Self {
        Self::new(vec![])
    }

    pub fn contains(&self, resource_id: &str) -> bool {
        self.resources.contains_key(resource_id)
    }

    pub fn get(&self, resource_id: &str) -> Option<&RbxResource> {
        self.resources.get(resource_id)
    }

    pub fn get_many(&self, resource_ids: Vec<&str>) -> Vec<&RbxResource> {
        resource_ids
            .iter()
            .filter_map(|id| self.resources.get(*id))
            .collect()
    }

    pub fn insert(&mut self, resource: RbxResource) {
        self.resources.insert(resource.id().to_owned(), resource);
    }

    // TODO: Can we make this less clone-y? Can we use actual resource references?
    pub fn topological_order(&self) -> anyhow::Result<Vec<&RbxResource>> {
        let mut dependency_graph: BTreeMap<String, Vec<String>> = self
            .resources
            .iter()
            .map(|(id, resource)| {
                (
                    id.clone(),
                    resource
                        .dependency_ids()
                        .iter()
                        .map(|d| d.to_owned().to_owned())
                        .collect(),
                )
            })
            .collect();

        let mut start_nodes: Vec<String> = dependency_graph
            .iter()
            .filter_map(|(node, deps)| {
                if deps.is_empty() {
                    Some(node.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut ordered: Vec<String> = Vec::new();
        while let Some(start_node) = start_nodes.pop() {
            ordered.push(start_node.clone());
            for (node, deps) in dependency_graph.iter_mut() {
                if deps.iter().any(|dep| *dep == start_node) {
                    deps.retain(|dep| *dep != start_node);
                    if deps.is_empty() {
                        start_nodes.push(node.clone());
                    }
                }
            }
        }

        let has_cycles = dependency_graph.iter().any(|(_, deps)| !deps.is_empty());
        match has_cycles {
            true => Err(anyhow::Error::msg(
                "Cannot evaluate resource graph because it has cycles",
            )),
            false => Ok(ordered
                .iter()
                .map(|id| self.resources.get(id).unwrap())
                .collect()),
        }
    }
}
