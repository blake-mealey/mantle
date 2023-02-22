use std::{
    fmt::Debug,
    sync::{Arc, RwLock, Weak},
};

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use self::{experience::ExperienceResource, place::PlaceResource};
use crate::resource_graph_v2::evaluator::ResourceGraphEvaluatorContext;

pub mod experience;
pub mod place;

// pub enum UpdateStrategy {
//     UpdateInPlace,
//     Recreate,
// }

pub trait ResourceInputs: Debug {}

pub trait ResourceOutputs: Debug {
    fn has_outputs(&self) -> bool;
}

pub type ResourceId = String;

pub type ResourceRef = Arc<RwLock<dyn ManagedResource>>;
pub type WeakResourceRef = Weak<RwLock<dyn ManagedResource>>;

#[enum_dispatch]
pub trait Resource: Debug {
    fn id(&self) -> &str;

    fn inputs(&self) -> &dyn ResourceInputs;

    fn outputs(&self) -> &dyn ResourceOutputs;

    fn dependencies(&self) -> Vec<WeakResourceRef>;

    // TODO: return simple update strategy enum here
}

#[async_trait]
#[enum_dispatch]
pub trait ManagedResource: Resource {
    // async fn creation_price(
    //     &self,
    //     context: &mut ResourceManagerContext,
    // ) -> anyhow::Result<Option<u32>> {
    //     Ok(None)
    // }

    // async fn create(
    //     &mut self,
    //     context: &mut ResourceManagerContext,
    //     price: Option<u32>,
    // ) -> anyhow::Result<()>;

    // // TODO: separate traits dependening on strategy
    // fn update_strategy(&self) -> UpdateStrategy {
    //     UpdateStrategy::UpdateInPlace
    // }

    // async fn update(
    //     &mut self,
    //     context: &mut ResourceManagerContext,
    //     price: Option<u32>,
    // ) -> anyhow::Result<()>;

    async fn delete(&mut self, context: &mut ResourceGraphEvaluatorContext) -> anyhow::Result<()>;

    async fn price(
        &mut self,
        context: &mut ResourceGraphEvaluatorContext,
    ) -> anyhow::Result<Option<u32>>;

    async fn create(
        &mut self,
        context: &mut ResourceGraphEvaluatorContext,
        price: Option<u32>,
    ) -> anyhow::Result<()>;

    fn update_strategy<'a>(&'a mut self) -> UpdateStrategy<'a>;
}

// TODO: simplify - just implement a noop update method for resources that use the recreate strategy
pub enum UpdateStrategy<'a> {
    Update(&'a mut dyn UpdatableResource),
    Recreate,
}

#[async_trait]
#[enum_dispatch]
pub trait UpdatableResource: ManagedResource {
    async fn update(&mut self, context: &mut ResourceGraphEvaluatorContext) -> anyhow::Result<()>;
}

#[enum_dispatch(Resource, ManagedResource)]
#[derive(Debug)]
pub enum ResourceDispatch {
    PlaceResource,
    ExperienceResource,
}

fn x(r: ResourceDispatch) {}
