use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
};

use crate::resources::{
    ExperienceInputs, ExperienceResource, PlaceInputs, PlaceResource, ResourceId, ResourceManager,
    ResourceManagerContext, ResourceOutputs, ResourceRef, ResourceVec, WeakResourceVec,
};

fn create_graph() {
    let experience = Rc::new(RefCell::new(ExperienceResource {
        id: "singleton".to_owned(),
        inputs: Box::new(ExperienceInputs { group_id: None }),
        outputs: None,
    }));

    let place = Rc::new(RefCell::new(PlaceResource {
        id: "start".to_owned(),
        inputs: Box::new(PlaceInputs { is_start: true }),
        outputs: None,
        experience: Rc::downgrade(&experience),
    }));

    let resources: ResourceVec = vec![experience, place];
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
                .map(|resource| (resource.borrow().id(), *resource))
                .collect(),
        }
    }

    pub fn outputs(&self, resource_id: &str) -> Option<Box<dyn ResourceOutputs>> {
        self.resources.get(resource_id).and_then(|resource| {
            let resource = resource.borrow();
            if resource.has_outputs() {
                Some(resource.outputs())
            } else {
                None
            }
        })
    }

    fn topological_order(&self) -> anyhow::Result<ResourceVec> {
        let mut dependency_graph: BTreeMap<ResourceId, WeakResourceVec> = self
            .resources
            .iter()
            .map(|(id, resource)| (id.clone(), resource.borrow().dependencies()))
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
                if deps.iter().any(|dep| dep.borrow().id() == start_node) {
                    deps.retain(|dep| dep.borrow().id() != start_node);
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

    pub async fn evaluate_delete(
        resource: Rc<RefCell<dyn ResourceManager>>,
        context: &mut ResourceManagerContext,
    ) -> anyhow::Result<()> {
        resource.borrow_mut().create(context, None);
        Ok(())
    }

    pub async fn evaluate(
        &self,
        previous_graph: &ResourceGraph,
        context: &ResourceManagerContext,
    ) -> anyhow::Result<()> {
        let mut previous_resources = previous_graph.topological_order()?;
        previous_resources.reverse();
        for resource in previous_resources {
            if self.resources.get(&resource.id()).is_some() {
                continue;
            }

            // TODO: delete
        }

        Ok(())
    }
}
