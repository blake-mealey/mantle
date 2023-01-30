use std::{
    borrow::Borrow,
    cell::RefCell,
    path::PathBuf,
    rc::{Rc, Weak},
};

use async_trait::async_trait;
use rbx_api::{
    experiences::models::CreateExperienceResponse,
    models::{AssetId, CreatorType},
    RobloxApi,
};

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

pub type ResourceRef = Rc<RefCell<dyn Resource>>;
pub type ResourceVec = Vec<ResourceRef>;
pub type WeakResourceRef = Weak<RefCell<dyn Resource>>;
pub type WeakResourceVec = Vec<WeakResourceRef>;

pub trait ResourceInputs {}

pub trait ResourceOutputs {}

pub type ResourceId = String;

pub trait Resource {
    fn id(&self) -> ResourceId;

    fn inputs(&self) -> Box<dyn ResourceInputs>;

    fn outputs(&self) -> Box<dyn ResourceOutputs>;

    fn has_outputs(&self) -> bool;

    fn dependencies(&self) -> WeakResourceVec;
}

#[async_trait]
pub trait ResourceManager: Resource {
    // async fn creation_price(
    //     &self,
    //     context: &mut ResourceManagerContext,
    // ) -> anyhow::Result<Option<u32>> {
    //     Ok(None)
    // }

    async fn create(
        &mut self,
        context: &mut ResourceManagerContext,
        price: Option<u32>,
    ) -> anyhow::Result<()>;

    // TODO: separate traits dependening on strategy
    fn update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::UpdateInPlace
    }

    async fn update(
        &mut self,
        context: &mut ResourceManagerContext,
        price: Option<u32>,
    ) -> anyhow::Result<()>;

    async fn delete(&mut self, context: &mut ResourceManagerContext) -> anyhow::Result<()>;
}

pub struct ExperienceResource {
    pub id: ResourceId,
    pub inputs: Box<ExperienceInputs>,
    pub outputs: Option<Box<ExperienceOutputs>>,
    pub something_else: bool,
}

pub struct ExperienceInputs {
    pub group_id: Option<AssetId>,
}
impl ResourceInputs for ExperienceInputs {}

pub struct ExperienceOutputs {
    pub asset_id: AssetId,
    pub start_place_id: AssetId,
}
impl ResourceOutputs for ExperienceOutputs {}

impl Resource for ExperienceResource {
    fn id(&self) -> ResourceId {
        self.id
    }

    fn inputs(&self) -> Box<dyn ResourceInputs> {
        self.inputs
    }

    fn has_outputs(&self) -> bool {
        self.outputs.is_some()
    }

    fn outputs(&self) -> Box<dyn ResourceOutputs> {
        self.outputs.unwrap()
    }

    fn dependencies(&self) -> WeakResourceVec {
        vec![]
    }
}

#[async_trait]
impl ResourceManager for ExperienceResource {
    async fn create(
        &mut self,
        context: &mut ResourceManagerContext,
        price: Option<u32>,
    ) -> anyhow::Result<()> {
        let CreateExperienceResponse {
            universe_id,
            root_place_id,
        } = context
            .roblox_api
            .create_experience(self.inputs.group_id)
            .await?;

        self.outputs = Some(Box::new(ExperienceOutputs {
            asset_id: universe_id,
            start_place_id: root_place_id,
        }));

        Ok(())
    }

    async fn update(
        &mut self,
        context: &mut ResourceManagerContext,
        price: Option<u32>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct PlaceResource {
    pub id: ResourceId,
    pub inputs: Box<PlaceInputs>,
    pub outputs: Option<Box<PlaceOutputs>>,
    pub experience: Weak<RefCell<ExperienceResource>>,
}

pub struct PlaceInputs {
    is_start: bool,
}
impl ResourceInputs for PlaceInputs {}

pub struct PlaceOutputs {
    pub asset_id: AssetId,
}
impl ResourceOutputs for PlaceOutputs {}

impl Resource for PlaceResource {
    fn id(&self) -> ResourceId {
        let experience = self.experience.upgrade().unwrap();
        experience.borrow().something_else;
        id
    }

    fn inputs(&self) -> Box<dyn ResourceInputs> {
        self.inputs
    }

    fn has_outputs(&self) -> bool {
        self.outputs.is_some()
    }

    fn outputs(&self) -> Box<dyn ResourceOutputs> {
        self.outputs.unwrap()
    }

    fn dependencies(&self) -> WeakResourceVec {
        vec![self.experience]
    }
}
