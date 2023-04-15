pub mod experience;
pub mod place;

use std::{fmt::Debug, vec};

use async_trait::async_trait;

use crate::resource_graph_v3::ResourceGraph;

use self::{experience::ExperienceResource, place::PlaceResource};

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Experience(ExperienceResource),
    Place(PlaceResource),
}

impl Resource {
    pub fn id(&self) -> &str {
        match self {
            Self::Experience(resource) => &resource.id,
            Self::Place(resource) => &resource.id,
        }
    }

    pub fn inputs(&self) -> &dyn Debug {
        match self {
            Self::Experience(resource) => &resource.inputs,
            Self::Place(resource) => &resource.inputs,
        }
    }

    pub fn outputs(&self) -> &dyn Debug {
        match self {
            Self::Experience(resource) => &resource.outputs,
            Self::Place(resource) => &resource.outputs,
        }
    }

    pub fn dependency_ids(&self) -> Vec<&str> {
        match self {
            Self::Experience(_resource) => vec![],
            Self::Place(resource) => vec![&resource.experience.id],
        }
    }

    pub fn next(&self, graph: &ResourceGraph) -> anyhow::Result<Resource> {
        match self {
            Self::Experience(resource) => Ok(Self::Experience(ExperienceResource {
                id: resource.id.clone(),
                inputs: resource.inputs.clone(),
                outputs: resource.outputs.clone(),
            })),
            Self::Place(resource) => Ok(Self::Place(PlaceResource {
                id: resource.id.clone(),
                inputs: resource.inputs.clone(),
                outputs: resource.outputs.clone(),
                experience: match graph
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
