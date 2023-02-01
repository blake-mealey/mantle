use std::sync::{RwLock, Weak};

use async_trait::async_trait;
use derive_resource::Resource;
use rbx_api::models::AssetId;

use crate::resource_graph_v2::evaluator::ResourceGraphEvaluatorContext;

use super::{
    experience::ExperienceResource, ManagedResource, Resource, ResourceId, ResourceInputs,
    ResourceOutputs, UpdateStrategy, WeakResourceRef,
};

#[derive(Debug)]
pub struct PlaceInputs {
    pub is_start: bool,
}
impl ResourceInputs for PlaceInputs {}

#[derive(Debug)]
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

#[derive(Resource, Debug)]
pub struct PlaceResource {
    pub id: ResourceId,
    pub inputs: PlaceInputs,
    pub outputs: PlaceOutputs,

    #[dependency]
    pub experience: Weak<RwLock<ExperienceResource>>,
}

#[async_trait]
impl ManagedResource for PlaceResource {
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
