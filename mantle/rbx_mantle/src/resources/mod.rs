use std::path::PathBuf;

use async_trait::async_trait;
use rbx_api::{models::CreatorType, RobloxApi};

pub mod experience;
pub mod place;

pub struct ResourceManagerContext {
    pub roblox_api: RobloxApi,
    pub project_path: PathBuf,
    pub payment_source: CreatorType,
    pub allow_purchases: bool,
}

pub enum UpdateStrategy {
    UpdateInPlace,
    Recreate,
}

pub trait ResourceInputs {}

pub trait ResourceOutputs {}

pub type ResourceId = String;

pub trait Resource<'a> {
    fn id(&self) -> ResourceId;

    fn inputs(&self) -> Box<dyn ResourceInputs>;

    fn outputs(&self) -> Option<Box<dyn ResourceOutputs>>;

    fn dependencies(&self) -> Vec<&'a dyn ManagedResource>;
}

#[async_trait]
pub trait ManagedResource<'a>: Resource<'a> {
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

    async fn delete(&mut self, context: &mut ResourceManagerContext) -> anyhow::Result<()>;
}
