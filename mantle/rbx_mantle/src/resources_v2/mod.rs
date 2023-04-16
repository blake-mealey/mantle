pub mod experience;
pub mod place;

use std::fmt::Debug;

use async_trait::async_trait;
use derive_resource::ResourceGroup;

use crate::resource_graph_v2::ResourceGraph;

use self::{experience::Experience, place::Place};

pub trait Resource: Sized {
    fn next(
        resource: &Self,
        previous_resource: Option<&RbxResource>,
        dependencies: Vec<&RbxResource>,
    ) -> anyhow::Result<Self>;

    fn dependency_ids(&self) -> Vec<&str>;
}

#[derive(Debug, Clone, PartialEq, ResourceGroup)]
pub enum RbxResource {
    Experience(Experience),
    Place(Place),
}

#[async_trait]
pub trait ResourceGroup {
    fn id(&self) -> &str;
    fn has_outputs(&self) -> bool;
    fn dependency_ids(&self) -> Vec<&str>;
    fn next(
        &self,
        previous_graph: &ResourceGraph,
        next_graph: &ResourceGraph,
    ) -> anyhow::Result<RbxResource>;

    // TODO: should these be separate somehow?
    async fn create(&mut self) -> anyhow::Result<()>;
    async fn update(&mut self) -> anyhow::Result<()>;
    async fn delete(&mut self) -> anyhow::Result<()>;
}

#[async_trait]
pub trait ManagedResource {
    async fn create(&mut self) -> anyhow::Result<()>;
    async fn update(&mut self) -> anyhow::Result<()>;
    async fn delete(&mut self) -> anyhow::Result<()>;
}
