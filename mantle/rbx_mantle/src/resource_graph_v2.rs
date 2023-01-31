use std::collections::BTreeMap;

use crate::resources::{
    experience::*, place::*, ManagedResource, ResourceId, ResourceManagerContext,
};

fn create_graph() {
    let mut experience = ExperienceResource {
        id: "singleton".to_owned(),
        inputs: ExperienceInputs { group_id: None },
        outputs: None,
    };

    let mut place = PlaceResource {
        id: "start".to_owned(),
        inputs: PlaceInputs { is_start: true },
        outputs: None,
        experience: &experience,
    };

    let resources: Vec<&mut dyn ManagedResource> = vec![&mut experience, &mut place];
    let graph = ResourceGraph::new(&resources);
}

pub struct ResourceGraph<'a> {
    resources: BTreeMap<ResourceId, &'a mut dyn ManagedResource<'a>>,
}

impl<'a> ResourceGraph<'a> {
    pub fn new(resources: &[&mut dyn ManagedResource]) -> Self {
        Self {
            resources: resources
                .iter()
                .map(|resource| (resource.id(), *resource))
                .collect(),
        }
    }

    fn topological_order(&self) -> anyhow::Result<Vec<&'a mut dyn ManagedResource>> {
        let mut dependency_graph: BTreeMap<ResourceId, Vec<&'a dyn ManagedResource>> = self
            .resources
            .iter()
            .map(|(id, resource)| (id.clone(), resource.dependencies()))
            .collect();

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
                if deps.iter().any(|dep| dep.id() == start_node) {
                    deps.retain(|dep| dep.id() != start_node);
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
                .map(|id| *self.resources.get(id).unwrap())
                .collect()),
        }
    }

    pub async fn evaluate_delete<'c>(
        &self,
        resource: &'c mut dyn ManagedResource<'c>,
        context: &mut ResourceManagerContext,
    ) -> anyhow::Result<()> {
        resource.delete(context).await
    }

    pub async fn evaluate<'b: 'a>(
        &'a self,
        previous_graph: &'b ResourceGraph<'b>,
        context: &mut ResourceManagerContext,
    ) -> anyhow::Result<()> {
        let mut previous_resources = previous_graph.topological_order()?;
        previous_resources.reverse();
        for resource in previous_resources {
            if self.resources.get(&resource.id()).is_some() {
                continue;
            }

            // TODO: delete
            self.evaluate_delete(resource, context);
        }

        let current_resources = self.topological_order()?;
        for resource in current_resources {
            self.evaluate_delete(resource, context);
        }

        Ok(())
    }
}
