use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use crate::resources::{experience::*, place::*, ResourceId, ResourceManagerContext, ResourceRef};

fn create_graph() {
    let experience = Arc::new(RwLock::new(ExperienceResource {
        id: "singleton".to_owned(),
        inputs: ExperienceInputs { group_id: None },
        outputs: ExperienceOutputs::Empty,
    }));

    let place = Arc::new(RwLock::new(PlaceResource {
        id: "start".to_owned(),
        inputs: PlaceInputs { is_start: true },
        outputs: PlaceOutputs::Empty,
        experience: Arc::downgrade(&experience),
    }));

    let resources: Vec<ResourceRef> = vec![experience, place];
    let graph = ResourceGraph::new(&resources);
}

pub struct ResourceGraph {
    resources: BTreeMap<ResourceId, ResourceRef>,
}

impl ResourceGraph {
    pub fn new(resources: &[ResourceRef]) -> Self {
        Self {
            resources: resources
                .iter()
                .map(|resource| {
                    (
                        resource.read().unwrap().id().to_owned(),
                        Arc::clone(resource),
                    )
                })
                .collect(),
        }
    }

    fn topological_order(&self) -> anyhow::Result<Vec<ResourceRef>> {
        let mut dependency_graph: BTreeMap<ResourceId, Vec<String>> = self
            .resources
            .iter()
            .map(|(id, resource)| {
                (
                    id.clone(),
                    resource
                        .read()
                        .unwrap()
                        .dependencies()
                        .iter()
                        .filter_map(|d| d.upgrade().map(|x| x.read().unwrap().id().to_owned()))
                        .collect(),
                )
            })
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
                .map(|id| Arc::clone(self.resources.get(id).unwrap()))
                .collect()),
        }
    }

    pub async fn evaluate_delete(
        &self,
        resource: ResourceRef,
        context: &mut ResourceManagerContext,
    ) -> anyhow::Result<()> {
        resource.write().unwrap().delete(context).await?;
        // .resource.delete(context).await;
        Ok(())
    }

    pub async fn evaluate(
        &self,
        previous_graph: &ResourceGraph,
        context: &mut ResourceManagerContext,
    ) -> anyhow::Result<()> {
        let mut previous_resources = previous_graph.topological_order()?;
        previous_resources.reverse();
        for resource in previous_resources {
            if self.resources.get(resource.read().unwrap().id()).is_some() {
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
