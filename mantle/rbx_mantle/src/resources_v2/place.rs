use async_trait::async_trait;
use rbx_api::models::AssetId;

use super::{experience::Experience, ManagedResource};

#[derive(Debug, Clone, PartialEq)]
pub struct PlaceInputs {
    pub is_start: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PlaceOutputs {
    pub asset_id: AssetId,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub id: String,
    pub inputs: PlaceInputs,
    pub outputs: Option<PlaceOutputs>,

    //#[dependency]
    pub experience: Experience,
}

#[async_trait]
impl ManagedResource for Place {
    async fn create(&mut self) -> anyhow::Result<()> {
        if self.inputs.is_start {
            self.outputs = Some(PlaceOutputs {
                asset_id: self.experience.outputs.as_ref().unwrap().start_place_id,
            })
        } else {
            self.outputs = Some(PlaceOutputs { asset_id: 3 });
        }
        Ok(())
    }
    async fn update(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    async fn delete(&mut self) -> anyhow::Result<()> {
        self.outputs = None;
        Ok(())
    }
}
