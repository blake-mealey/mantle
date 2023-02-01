use async_trait::async_trait;
use derive_resource::Resource;
use rbx_api::models::AssetId;

use crate::resource_graph_v2::evaluator::ResourceGraphEvaluatorContext;

use super::{
    ManagedResource, Resource, ResourceId, ResourceInputs, ResourceOutputs, UpdateStrategy,
    WeakResourceRef,
};

#[derive(Debug)]
pub struct ExperienceInputs {
    pub group_id: Option<AssetId>,
}
impl ResourceInputs for ExperienceInputs {}

#[derive(Debug)]
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

#[derive(Resource, Debug)]
pub struct ExperienceResource {
    pub id: ResourceId,
    pub inputs: ExperienceInputs,
    pub outputs: ExperienceOutputs,
}

#[async_trait]
impl ManagedResource for ExperienceResource {
    async fn delete(&mut self, _context: &mut ResourceGraphEvaluatorContext) -> anyhow::Result<()> {
        todo!()
    }

    async fn price(
        &mut self,
        _context: &mut ResourceGraphEvaluatorContext,
    ) -> anyhow::Result<Option<u32>> {
        todo!()
    }

    async fn create(
        &mut self,
        _context: &mut ResourceGraphEvaluatorContext,
        _price: Option<u32>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn update_strategy<'a>(&'a mut self) -> UpdateStrategy<'a> {
        UpdateStrategy::Recreate
    }
}
