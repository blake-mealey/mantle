pub mod experience;
pub mod place;

use std::{fmt::Debug, vec};

use async_trait::async_trait;

use crate::resource_graph_v3::ResourceGraph;

use self::{experience::Experience, place::Place};

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Experience(Experience),
    Place(Place),
}

impl Resource {
    pub fn id(&self) -> &str {
        match self {
            Self::Experience(resource) => &resource.id,
            Self::Place(resource) => &resource.id,
        }
    }

    pub fn has_outputs(&self) -> bool {
        match self {
            Self::Experience(resource) => resource.outputs.is_some(),
            Self::Place(resource) => resource.outputs.is_some(),
        }
    }

    pub fn dependency_ids(&self) -> Vec<&str> {
        match self {
            Self::Experience(_resource) => vec![],
            Self::Place(resource) => vec![&resource.experience.id],
        }
    }

    pub fn next(
        &self,
        previous_graph: &ResourceGraph,
        next_graph: &ResourceGraph,
    ) -> anyhow::Result<Resource> {
        match self {
            Self::Experience(resource) => Ok(Self::Experience(Experience {
                id: resource.id.clone(),
                inputs: resource.inputs.clone(),
                outputs: match previous_graph.get(&resource.id) {
                    Some(previous_resource) => match previous_resource {
                        Resource::Experience(previous_resource) => {
                            previous_resource.outputs.clone()
                        }
                        _ => {
                            return anyhow::Result::Err(anyhow::Error::msg("Expected 'experience'"))
                        }
                    },
                    _ => None,
                },
            })),
            Self::Place(resource) => Ok(Self::Place(Place {
                id: resource.id.clone(),
                inputs: resource.inputs.clone(),
                outputs: match previous_graph.get(&resource.id) {
                    Some(previous_resource) => match previous_resource {
                        Resource::Place(previous_resource) => previous_resource.outputs.clone(),
                        _ => return anyhow::Result::Err(anyhow::Error::msg("Expected 'place'")),
                    },
                    _ => None,
                },
                experience: match next_graph
                    .get(&resource.experience.id)
                    .ok_or(anyhow::Error::msg("Unable to find resource"))?
                {
                    Resource::Experience(experience) => experience.clone(),
                    _ => return anyhow::Result::Err(anyhow::Error::msg("Expected 'experience'")),
                },
            })),
        }
    }

    pub async fn create(&mut self) -> anyhow::Result<()> {
        match self {
            Self::Experience(resource) => resource.create().await,
            Self::Place(resource) => resource.create().await,
        }
    }

    pub async fn update(&mut self) -> anyhow::Result<()> {
        match self {
            Self::Experience(resource) => resource.update().await,
            Self::Place(resource) => resource.update().await,
        }
    }

    pub async fn delete(&mut self) -> anyhow::Result<()> {
        match self {
            Self::Experience(resource) => resource.delete().await,
            Self::Place(resource) => resource.delete().await,
        }
    }
}

#[async_trait]
pub trait ManagedResource {
    async fn create(&mut self) -> anyhow::Result<()>;
    async fn update(&mut self) -> anyhow::Result<()>;
    async fn delete(&mut self) -> anyhow::Result<()>;
}
