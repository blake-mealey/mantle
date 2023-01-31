use async_trait::async_trait;
use derive_resource::Resource;
use rbx_api::models::AssetId;

use super::{
    ManagedResource, Resource, ResourceId, ResourceInputs, ResourceManagerContext, ResourceOutputs,
    WeakResourceRef,
};

pub struct ExperienceInputs {
    pub group_id: Option<AssetId>,
}
impl ResourceInputs for ExperienceInputs {}

pub enum ExperienceOutputs {
    Data {
        asset_id: AssetId,
        start_place_id: AssetId,
    },
    Empty,
}
impl ResourceOutputs for ExperienceOutputs {
    fn has_outputs(&self) -> bool {
        match self {
            Self::Empty => false,
            _ => true,
        }
    }
}

#[derive(Resource)]
pub struct ExperienceResource {
    pub id: ResourceId,
    pub inputs: ExperienceInputs,
    pub outputs: ExperienceOutputs,
}

#[async_trait]
impl ManagedResource for ExperienceResource {
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
