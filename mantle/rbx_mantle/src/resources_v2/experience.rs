use async_trait::async_trait;
use rbx_api::models::AssetId;

use super::ManagedResource;

#[derive(Debug, Clone, PartialEq)]
pub struct ExperienceInputs {
    pub group_id: Option<AssetId>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ExperienceOutputs {
    pub asset_id: AssetId,
    pub start_place_id: AssetId,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Experience {
    pub id: String,
    pub inputs: ExperienceInputs,
    pub outputs: Option<ExperienceOutputs>,
}

#[async_trait]
impl ManagedResource for Experience {
    async fn create(&mut self) -> anyhow::Result<()> {
        self.outputs = Some(ExperienceOutputs {
            asset_id: 1,
            start_place_id: 2,
        });
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
