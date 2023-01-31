use std::{
    path::PathBuf,
    sync::{Arc, RwLock, Weak},
};

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

pub trait ResourceOutputs {
    fn has_outputs(&self) -> bool;
}

pub type ResourceId = String;

pub type ResourceRef = Arc<RwLock<dyn ManagedResource>>;
pub type WeakResourceRef = Weak<RwLock<dyn ManagedResource>>;

pub trait Resource {
    fn id(&self) -> &str;

    fn inputs(&self) -> &dyn ResourceInputs;

    fn outputs(&self) -> &dyn ResourceOutputs;

    fn dependencies(&self) -> Vec<WeakResourceRef>;
}

#[async_trait]
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

    async fn delete(&mut self, context: &mut ResourceManagerContext) -> anyhow::Result<()>;
}
