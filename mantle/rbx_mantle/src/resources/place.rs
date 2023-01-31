use async_trait::async_trait;
use rbx_api::models::AssetId;

use super::{
    experience::ExperienceResource, ManagedResource, Resource, ResourceId, ResourceInputs,
    ResourceManagerContext, ResourceOutputs,
};

pub struct PlaceResource<'a> {
    pub id: ResourceId,
    pub inputs: PlaceInputs,
    pub outputs: Option<PlaceOutputs>,
    pub experience: &'a ExperienceResource,
}

pub struct PlaceInputs {
    is_start: bool,
}
impl ResourceInputs for PlaceInputs {}

pub struct PlaceOutputs {
    pub asset_id: AssetId,
}
impl ResourceOutputs for PlaceOutputs {}

impl<'a> Resource<'a> for PlaceResource<'a> {
    fn id(&self) -> ResourceId {
        self.id
    }

    fn inputs(&self) -> Box<dyn ResourceInputs> {
        Box::new(self.inputs)
    }

    fn outputs(&self) -> Option<Box<dyn ResourceOutputs>> {
        self.outputs
            .map(|o| Box::new(o) as Box<dyn ResourceOutputs>)
    }

    fn dependencies(&self) -> Vec<&'a dyn ManagedResource> {
        vec![self.experience]
    }
}

#[async_trait]
impl<'a> ManagedResource<'a> for PlaceResource<'a> {
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
