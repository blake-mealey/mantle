use async_trait::async_trait;
use rbx_api::models::AssetId;

use super::{
    ManagedResource, Resource, ResourceId, ResourceInputs, ResourceManagerContext, ResourceOutputs,
};

pub struct ExperienceResource {
    pub id: ResourceId,
    pub inputs: ExperienceInputs,
    pub outputs: Option<ExperienceOutputs>,
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

impl<'a> Resource<'a> for ExperienceResource {
    fn id(&self) -> ResourceId {
        self.id
    }

    // TODO: Should this be a Box? Is there a better container for it?
    fn inputs(&self) -> Box<dyn ResourceInputs> {
        Box::new(self.inputs)
    }

    fn outputs(&self) -> Option<Box<dyn ResourceOutputs>> {
        self.outputs
            .map(|o| Box::new(o) as Box<dyn ResourceOutputs>)
    }

    fn dependencies(&self) -> Vec<&'a dyn ManagedResource> {
        vec![]
    }
}

#[async_trait]
impl<'a> ManagedResource<'a> for ExperienceResource {
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
