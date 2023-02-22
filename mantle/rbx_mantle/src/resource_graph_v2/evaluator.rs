use std::path::PathBuf;

use rbx_api::{models::CreatorType, RobloxApi};
use rbx_auth::RobloxAuth;

use super::ResourceGraph;
use crate::resources::{ResourceId, ResourceRef, UpdateStrategy};

pub struct ResourceGraphEvaluatorOptions {
    pub project_path: PathBuf,
    pub payment_source: CreatorType,
    pub allow_purchases: bool,
}

pub struct ResourceGraphEvaluatorContext {
    pub options: ResourceGraphEvaluatorOptions,
    pub roblox_api: RobloxApi,
}

pub enum SkipReason {
    PurchasesNotAllowed,
}

pub enum OperationType {
    Create,
    Update,
    Recreate,
    Delete,
    Noop,
    Skip(SkipReason),
}

pub enum OperationStatus {
    Success,
    Failure(anyhow::Error),
}

pub struct OperationResult {
    pub resource_id: ResourceId,
    pub operation_type: OperationType,
    pub status: OperationStatus,
}

#[derive(Default)]
pub struct EvaluatorResults {
    pub operation_results: Vec<OperationResult>,
}

impl EvaluatorResults {
    pub fn is_empty(&self) -> bool {
        self.operation_results.is_empty()
    }

    pub fn create_succeeded(&mut self, resource_id: ResourceId) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Create,
            status: OperationStatus::Success,
        })
    }
    pub fn create_failed(&mut self, resource_id: ResourceId, error: anyhow::Error) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Create,
            status: OperationStatus::Failure(error),
        })
    }

    pub fn update_succeeded(&mut self, resource_id: ResourceId) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Update,
            status: OperationStatus::Success,
        })
    }
    pub fn update_failed(&mut self, resource_id: ResourceId, error: anyhow::Error) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Update,
            status: OperationStatus::Failure(error),
        })
    }

    pub fn recreate_succeeded(&mut self, resource_id: ResourceId) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Recreate,
            status: OperationStatus::Success,
        })
    }
    pub fn recreate_failed(&mut self, resource_id: ResourceId, error: anyhow::Error) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Recreate,
            status: OperationStatus::Failure(error),
        })
    }

    pub fn delete_succeeded(&mut self, resource_id: ResourceId) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Delete,
            status: OperationStatus::Success,
        })
    }
    pub fn delete_failed(&mut self, resource_id: ResourceId, error: anyhow::Error) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Delete,
            status: OperationStatus::Failure(error),
        })
    }

    pub fn noop(&mut self, resource_id: ResourceId) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Noop,
            status: OperationStatus::Success,
        })
    }

    pub fn skip(&mut self, resource_id: ResourceId, reason: SkipReason) {
        self.operation_results.push(OperationResult {
            resource_id,
            operation_type: OperationType::Skip(reason),
            status: OperationStatus::Success,
        })
    }
}

pub struct ResouceGraphEvaluator<'a> {
    context: ResourceGraphEvaluatorContext,
    previous_graph: &'a ResourceGraph,
    desired_graph: &'a ResourceGraph,
    next_graph: ResourceGraph,
    results: EvaluatorResults,
}

impl<'a> ResouceGraphEvaluator<'a> {
    pub async fn new(
        options: ResourceGraphEvaluatorOptions,
        previous_graph: &'a ResourceGraph,
        current_graph: &'a ResourceGraph,
    ) -> anyhow::Result<ResouceGraphEvaluator<'a>> {
        let roblox_auth = RobloxAuth::new().await?;
        let roblox_api = RobloxApi::new(roblox_auth)?;
        Ok(ResouceGraphEvaluator {
            context: ResourceGraphEvaluatorContext {
                options,
                roblox_api,
            },
            previous_graph,
            desired_graph: current_graph,
            next_graph: ResourceGraph::default(),
            results: EvaluatorResults::default(),
        })
    }

    pub async fn evaluate(
        &'a mut self,
    ) -> anyhow::Result<(&'a EvaluatorResults, &'a ResourceGraph)> {
        if !self.results.is_empty() {
            panic!("Cannot use a graph evaluator more than once");
        }

        self.delete_removed_resources().await?;
        self.create_or_update_added_or_changed_resources().await?;

        Ok((&self.results, &self.next_graph))
    }

    async fn delete_removed_resources(&mut self) -> anyhow::Result<()> {
        let mut previous_resources = self.previous_graph.topological_order()?;

        // Iterate over previous resources in reverse order so that leaf resources are removed first
        previous_resources.reverse();

        for resource in previous_resources.iter() {
            // no need to delete resources that still exist
            let mut resource_write_ref = resource.write().unwrap();
            let resource_id = resource_write_ref.id().to_owned();
            if self.desired_graph.contains(&resource_id) {
                continue;
            }

            println!("Deleting: {}", resource_id);
            // TODO: diff
            println!("Dependencies: {:?}", resource_write_ref.dependencies());
            println!("Inputs: {:?}", resource_write_ref.inputs());

            let delete_result = resource_write_ref.delete(&mut self.context).await;

            match delete_result {
                Ok(()) => {
                    println!("Deleted resource {}", resource_id);
                    self.results.delete_succeeded(resource_id);
                }
                Err(error) => {
                    println!("Failed to delete resource {}: {}", resource_id, error);
                    self.results.delete_failed(resource_id, error);
                }
            }
        }

        Ok(())
    }

    async fn create_resource(&mut self, resource: ResourceRef) {
        let mut resource_write_ref = resource.write().unwrap();
        let resource_id = resource_write_ref.id().to_owned();

        println!("Creating: {}", resource_id);
        // TODO: diff
        println!("Dependencies: {:?}", resource_write_ref.dependencies());
        println!("Inputs: {:?}", resource_write_ref.inputs());

        let create_price_result = resource_write_ref.price(&mut self.context).await;

        let price = match create_price_result {
            Ok(Some(price)) if price > 0 => {
                if self.context.options.allow_purchases {
                    println!("{} Robux will be charged from your account.", price);
                    Some(price)
                } else {
                    self.results
                        .skip(resource_id, SkipReason::PurchasesNotAllowed);
                    println!("Resource would cost {} to create. Give Mantle permission to make purchases with --allow-purchases.", price);
                    return;
                }
            }
            Ok(_) => None,
            Err(error) => {
                self.results.create_failed(resource_id, error);
                return;
            }
        };

        let create_result = resource_write_ref.create(&mut self.context, price).await;

        match create_result {
            Ok(()) => {
                println!(
                    "Created resource {} with outputs: {:?}",
                    resource_id,
                    resource_write_ref.outputs()
                );
                self.next_graph.insert(&resource_id, resource.clone());
                self.results.create_succeeded(resource_id);
            }
            Err(error) => {
                println!("Failed to create resource {}: {}", resource_id, error);
                self.results.create_failed(resource_id, error);
            }
        }
    }

    fn update_resource(&mut self, resource: ResourceRef) {
        let mut resource_write_ref = resource.write().unwrap();
        let resource_id = resource_write_ref.id().to_owned();

        // match resource.write().unwrap().update_strategy() {
        //     UpdateStrategy::Recreate => {
        //         self.create_resource(resource.clone()).await;
        //         return;
        //     }
        //     UpdateStrategy::Update(updatable_resource) => {
        //         updatable_resource.update(context);
        //     }
        // }

        println!("Updating: {}", resource_id);
    }

    async fn create_or_update_added_or_changed_resources(&mut self) -> anyhow::Result<()> {
        let resources = self.desired_graph.topological_order()?;

        for resource in resources.iter() {
            let resource_read_ref = resource.read().unwrap();
            let resource_id = resource_read_ref.id().to_owned();
            drop(resource_read_ref);

            if let Some(previous_resource) = self.previous_graph.get(&resource_id) {
                // compare, check strategy, update or recreate
            } else {
                self.create_resource(resource.clone()).await;
            }
        }

        Ok(())
    }
}
