use std::sync::{RwLock, Weak};

use async_trait::async_trait;
use derive_resource::Resource;
use rbx_api::models::AssetId;

use super::{
    experience::ExperienceResource, ManagedResource, Resource, ResourceId, ResourceInputs,
    ResourceManagerContext, ResourceOutputs, WeakResourceRef,
};

pub struct PlaceInputs {
    pub is_start: bool,
}
impl ResourceInputs for PlaceInputs {}

pub enum PlaceOutputs {
    Data { asset_id: AssetId },
    Empty,
}
impl ResourceOutputs for PlaceOutputs {
    fn has_outputs(&self) -> bool {
        match self {
            Self::Empty => false,
            _ => true,
        }
    }
}

#[derive(Resource)]
pub struct PlaceResource {
    pub id: ResourceId,
    pub inputs: PlaceInputs,
    pub outputs: PlaceOutputs,

    #[dependency]
    pub experience: Weak<RwLock<ExperienceResource>>,
}

#[async_trait]
impl ManagedResource for PlaceResource {
    // async fn create(
    //     &mut self,
    //     context: &mut ResourceManagerContext,
    //     price: Option<u32>,
    // ) -> anyhow::Result<()> {
    //     let CreateExperienceResponse {
    //         universe_id,
    //         root_place_id,
    //     } = context
    //         .roblox_api
    //         .create_experience(self.inputs.group_id)
    //         .await?;

    //     self.outputs = Some(Box::new(ExperienceOutputs {
    //         asset_id: universe_id,
    //         start_place_id: root_place_id,
    //     }));

    //     Ok(())
    // }

    // async fn update(
    //     &mut self,
    //     context: &mut ResourceManagerContext,
    //     price: Option<u32>,
    // ) -> anyhow::Result<()> {
    //     Ok(())
    // }

    async fn delete(&mut self, context: &mut ResourceManagerContext) -> anyhow::Result<()> {
        Ok(())
    }
}
