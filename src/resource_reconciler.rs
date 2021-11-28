use std::collections::HashMap;

use async_recursion::async_recursion;
use difference::Changeset;
use serde::Serialize;
use yansi::Paint;

use crate::{
    logger,
    resource_graph::{Resource, ResourceGraph, ResourceId, ResourceManager, UpdateType},
};

pub struct ResourceReconcilerOptions {
    allow_purchases: bool,
}

pub struct ReconcileResults<TResource, TInputs, TOutputs>
where
    TResource: Resource<TInputs, TOutputs>,
    TInputs: Clone,
    TOutputs: Clone,
    TOutputs: Serialize,
{
    pub created_count: u32,
    pub updated_count: u32,
    pub deleted_count: u32,
    pub noop_count: u32,
    pub skipped_count: u32,

    pub failed_count: u32,

    pub next_graph: ResourceGraph<TResource, TInputs, TOutputs>,
}

enum PreviousResourceOperation {
    Delete { resource_id: ResourceId },
    Noop,
}

enum PreviousResourceOperationResult {
    Succeeded,
    Failed(String),
}

// TODO: include required data (resource, etc)
#[derive(Clone)]
enum CurrentResourceOperation {
    Create {
        resource_id: ResourceId,
    },
    UpdateInPlace {
        resource_id: ResourceId,
    },
    Recreate {
        resource_id: ResourceId,
        delete_first: bool,
    },
    UpdateOrNoop {
        resource_id: ResourceId,
    },
    Noop,
}

enum CurrentResourceOperationResult<TOutputs>
where
    TOutputs: Clone,
    TOutputs: Serialize,
{
    SucceededNoOutputs,
    Succeeded(TOutputs),
    Failed(String),
    MissingDependencies,
    Skipped(String),
}

fn get_changeset(previous_hash: &str, new_hash: &str) -> Changeset {
    Changeset::new(previous_hash, new_hash, "\n")
}

pub struct ResourceReconciler<TResource, TInputs, TOutputs, TManager>
where
    TResource: Resource<TInputs, TOutputs>,
    TInputs: Clone,
    TOutputs: Clone,
    TOutputs: Serialize,
    TManager: ResourceManager<TInputs, TOutputs>,
{
    previous_graph: ResourceGraph<TResource, TInputs, TOutputs>,
    current_graph: ResourceGraph<TResource, TInputs, TOutputs>,
    manager: TManager,
    options: ResourceReconcilerOptions,
}

impl<TResource, TInputs, TOutputs, TManager>
    ResourceReconciler<TResource, TInputs, TOutputs, TManager>
where
    TResource: Resource<TInputs, TOutputs>,
    TInputs: Clone,
    TOutputs: Clone,
    TOutputs: Serialize,
    TManager: ResourceManager<TInputs, TOutputs>,
{
    pub fn new(
        previous_graph: ResourceGraph<TResource, TInputs, TOutputs>,
        current_graph: ResourceGraph<TResource, TInputs, TOutputs>,
        manager: TManager,
        options: ResourceReconcilerOptions,
    ) -> Self {
        Self {
            previous_graph,
            current_graph,
            manager,
            options,
        }
    }

    fn get_previous_resource_operation(
        &self,
        resource_id: ResourceId,
    ) -> PreviousResourceOperation {
        let current_resource = self.current_graph.get_resource(&resource_id);

        if current_resource.is_some() {
            return PreviousResourceOperation::Noop;
        }

        return PreviousResourceOperation::Delete { resource_id };
    }

    fn get_current_resource_operation(
        &self,
        resource_id: ResourceId,
        operations_map: &HashMap<ResourceId, CurrentResourceOperation>,
    ) -> CurrentResourceOperation {
        let current_resource = self.current_graph.get_resource(&resource_id).unwrap();
        let previous_resource = self.previous_graph.get_resource(&resource_id);

        let previous_resource = match previous_resource {
            Some(previous_resource) => previous_resource,
            None => {
                return CurrentResourceOperation::Create { resource_id };
            }
        };

        let current_dependency_outputs =
            self.current_graph.get_dependency_outputs(current_resource);
        if let Some(current_dependency_outputs) = current_dependency_outputs {
            let current_inputs_hash = current_resource.get_inputs_hash();
            let current_dependencies_hash =
                ResourceGraph::<TResource, TInputs, TOutputs>::get_dependency_outputs_hash(
                    current_dependency_outputs,
                );

            let previous_inputs_hash = previous_resource.get_inputs_hash();
            let previous_dependency_outputs = self
                .previous_graph
                .get_dependency_outputs(previous_resource)
                .expect("Previous graph should be complete.");
            let previous_dependencies_hash =
                ResourceGraph::<TResource, TInputs, TOutputs>::get_dependency_outputs_hash(
                    previous_dependency_outputs,
                );

            if current_inputs_hash == previous_inputs_hash
                && current_dependencies_hash == previous_dependencies_hash
            {
                return CurrentResourceOperation::Noop;
            }

            let update_type = self.manager.get_update_type(current_resource.get_inputs());
            match update_type {
                UpdateType::UpdateInPlace => {
                    return CurrentResourceOperation::UpdateInPlace { resource_id };
                }
                UpdateType::Recreate { delete_first } => {
                    return CurrentResourceOperation::Recreate {
                        resource_id,
                        delete_first,
                    };
                }
            }
        } else {
            let dependency_operations = current_resource
                .get_dependencies()
                .iter()
                .map(|id| operations_map.get(id).unwrap())
                .collect::<Vec<_>>();

            if dependency_operations.iter().any(|op| {
                matches!(
                    op,
                    CurrentResourceOperation::Create { .. }
                        | CurrentResourceOperation::Recreate { .. }
                )
            }) {
                let update_type = self.manager.get_update_type(current_resource.get_inputs());
                match update_type {
                    UpdateType::UpdateInPlace => {
                        return CurrentResourceOperation::UpdateInPlace { resource_id };
                    }
                    UpdateType::Recreate { delete_first } => {
                        return CurrentResourceOperation::Recreate {
                            resource_id,
                            delete_first,
                        };
                    }
                }
            } else {
                return CurrentResourceOperation::UpdateOrNoop { resource_id };
            }
        }
    }

    async fn evaluate_previous_resource_operation(
        &self,
        operation: &PreviousResourceOperation,
    ) -> PreviousResourceOperationResult {
        match operation {
            PreviousResourceOperation::Delete { resource_id } => {
                let resource = self.previous_graph.get_resource(resource_id).unwrap();
                let dependency_outputs = self
                    .previous_graph
                    .get_dependency_outputs(resource)
                    .expect("Previous graph should be complete.");
                match self
                    .manager
                    .delete(
                        resource
                            .get_outputs()
                            .expect("Existing resource should have outputs."),
                        dependency_outputs,
                    )
                    .await
                {
                    Ok(_) => PreviousResourceOperationResult::Succeeded,
                    Err(error) => PreviousResourceOperationResult::Failed(error),
                }
            }
            PreviousResourceOperation::Noop => PreviousResourceOperationResult::Succeeded,
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_current_resource_operation(
        &self,
        operation: &CurrentResourceOperation,
    ) -> CurrentResourceOperationResult<TOutputs> {
        match operation {
            CurrentResourceOperation::Create { resource_id } => {
                let resource = self.current_graph.get_resource(resource_id).unwrap();
                let inputs = resource.get_inputs();

                let dependency_outputs = match self.current_graph.get_dependency_outputs(resource) {
                    Some(v) => v,
                    None => {
                        // TODO: We can print the exact dependency name which is missing here
                        return CurrentResourceOperationResult::MissingDependencies;
                    }
                };

                match self
                    .manager
                    .get_create_price(inputs.clone(), dependency_outputs.clone())
                    .await
                {
                    Ok(Some(price)) => {
                        if self.options.allow_purchases {
                            // TODO: log warning
                        } else {
                            return CurrentResourceOperationResult::Skipped(format!("Resource would cost {} Robux to create. Give Mantle permission to make purchases with --allow-purchases.", price));
                        }
                    }
                    Err(error) => return CurrentResourceOperationResult::Failed(error),
                    Ok(_) => {}
                };

                match self.manager.create(inputs, dependency_outputs).await {
                    Ok(outputs) => CurrentResourceOperationResult::Succeeded(outputs),
                    Err(error) => CurrentResourceOperationResult::Failed(error),
                }
            }
            CurrentResourceOperation::UpdateInPlace { resource_id } => {
                let resource = self.current_graph.get_resource(resource_id).unwrap();
                let inputs = resource.get_inputs();
                let outputs = resource
                    .get_outputs()
                    .expect("Existing resource should have outputs.");

                let dependency_outputs = match self.current_graph.get_dependency_outputs(resource) {
                    Some(v) => v,
                    None => {
                        // TODO: We can print the exact dependency name which is missing here
                        return CurrentResourceOperationResult::MissingDependencies;
                    }
                };

                match self
                    .manager
                    .update(inputs, outputs, dependency_outputs)
                    .await
                {
                    Ok(outputs) => CurrentResourceOperationResult::Succeeded(outputs),
                    Err(error) => CurrentResourceOperationResult::Failed(error),
                }
            }
            CurrentResourceOperation::Recreate {
                resource_id,
                delete_first,
            } => {
                if *delete_first {
                    let delete_result = self
                        .evaluate_previous_resource_operation(&PreviousResourceOperation::Delete {
                            resource_id: resource_id.to_owned(),
                        })
                        .await;

                    match delete_result {
                        PreviousResourceOperationResult::Succeeded => {}
                        PreviousResourceOperationResult::Failed(reason) => {
                            return CurrentResourceOperationResult::Failed(reason);
                        }
                    };
                }

                // TODO: Do we need another state in the case it deletes but fails to create?
                self.evaluate_current_resource_operation(&CurrentResourceOperation::Create {
                    resource_id: resource_id.to_owned(),
                })
                .await
            }
            CurrentResourceOperation::UpdateOrNoop { resource_id } => {
                let new_operation =
                    self.get_current_resource_operation(resource_id.clone(), &HashMap::new());

                if matches!(new_operation, CurrentResourceOperation::UpdateOrNoop { .. }) {
                    return CurrentResourceOperationResult::MissingDependencies;
                }

                self.evaluate_current_resource_operation(&new_operation)
                    .await
            }
            CurrentResourceOperation::Noop => CurrentResourceOperationResult::SucceededNoOutputs,
        }
    }

    fn handle_previous_operation_result(
        &self,
        operation: &PreviousResourceOperation,
        result: &PreviousResourceOperationResult,
        reconcile_results: &mut ReconcileResults<TResource, TInputs, TOutputs>,
    ) {
        match operation {
            PreviousResourceOperation::Delete { resource_id } => {
                let resource = self.previous_graph.get_resource(resource_id).unwrap();
                let dependency_outputs = self
                    .previous_graph
                    .get_dependency_outputs(resource)
                    .expect("Previous graph should be complete.");
                let dependencies_hash =
                    ResourceGraph::<TResource, TInputs, TOutputs>::get_dependency_outputs_hash(
                        dependency_outputs,
                    );
                let inputs_hash = resource.get_inputs_hash();

                logger::start_action(format!("{} Deleting: {}", Paint::red("-"), resource_id));
                logger::log("Dependencies:");
                logger::log_changeset(get_changeset(&dependencies_hash, &dependencies_hash));
                logger::log("Inputs:");
                logger::log_changeset(get_changeset(&inputs_hash, ""));

                match result {
                    PreviousResourceOperationResult::Succeeded => {
                        // No need to update the graph since it's already not present
                        reconcile_results.deleted_count += 1;

                        logger::end_action_with_results(
                            "Succeeded with outputs:",
                            get_changeset(&resource.get_outputs_hash(), ""),
                        );
                    }
                    PreviousResourceOperationResult::Failed(reason) => {
                        // We failed to delete the resource, so we need to add the previous resource
                        // to the next graph
                        reconcile_results
                            .next_graph
                            .insert_resource(resource.to_owned());
                        reconcile_results.failed_count += 1;

                        logger::end_action(format!("Failed: {}", Paint::red(reason)));
                    }
                }
            }
            PreviousResourceOperation::Noop => {}
        }
    }

    fn handle_current_operation_result(
        &self,
        operation: &CurrentResourceOperation,
        result: &CurrentResourceOperationResult<TOutputs>,
        reconcile_results: &mut ReconcileResults<TResource, TInputs, TOutputs>,
    ) {
        match operation {
            CurrentResourceOperation::Create { resource_id } => {
                let resource = self.current_graph.get_resource(resource_id).unwrap();
                let inputs_hash = resource.get_inputs_hash();

                logger::start_action(format!("{} Creating: {}", Paint::green("+"), resource_id));

                if matches!(result, CurrentResourceOperationResult::MissingDependencies) {
                    // No need to update the graph since it's already not present
                    reconcile_results.skipped_count += 1;

                    logger::end_action(format!(
                        "Skipped: {}",
                        Paint::yellow("A dependency failed to produce outputs.")
                    ));
                    return;
                }

                let dependency_outputs =
                    self.current_graph.get_dependency_outputs(resource).unwrap();
                let dependencies_hash =
                    ResourceGraph::<TResource, TInputs, TOutputs>::get_dependency_outputs_hash(
                        dependency_outputs,
                    );

                logger::log("Dependencies:");
                logger::log_changeset(get_changeset(&dependencies_hash, &dependencies_hash));
                logger::log("Inputs:");
                logger::log_changeset(get_changeset("", &inputs_hash));

                match result {
                    CurrentResourceOperationResult::Succeeded(outputs) => {
                        // Add the next resource with its outputs to the next graph
                        let mut next_resource = resource.clone();
                        next_resource.set_outputs(outputs.to_owned());
                        reconcile_results.next_graph.insert_resource(next_resource);
                        reconcile_results.created_count += 1;

                        logger::end_action_with_results(
                            "Succeeded with outputs:",
                            get_changeset("", &resource.get_outputs_hash()),
                        );
                    }
                    CurrentResourceOperationResult::Failed(reason) => {
                        // No need to update the graph since it's already not present
                        reconcile_results.failed_count += 1;

                        logger::end_action(format!("Failed: {}", Paint::red(reason)));
                    }
                    CurrentResourceOperationResult::Skipped(reason) => {
                        // No need to update the graph since it's already not present
                        reconcile_results.skipped_count += 1;

                        logger::end_action(format!("Skipped: {}", Paint::yellow(reason)));
                    }
                    _ => unreachable!(),
                }
            }
            CurrentResourceOperation::UpdateInPlace { resource_id } => {
                let resource = self.current_graph.get_resource(resource_id).unwrap();
                let inputs_hash = resource.get_inputs_hash();

                let previous_resource = self.previous_graph.get_resource(resource_id).unwrap();
                let previous_hash = previous_resource.get_inputs_hash();
                let previous_dependency_outputs = self
                    .previous_graph
                    .get_dependency_outputs(previous_resource)
                    .unwrap();
                let previous_dependencies_hash =
                    ResourceGraph::<TResource, TInputs, TOutputs>::get_dependency_outputs_hash(
                        previous_dependency_outputs,
                    );

                logger::start_action(format!("{} Updating: {}", Paint::yellow("~"), resource_id));

                if matches!(result, CurrentResourceOperationResult::MissingDependencies) {
                    // No need to update the graph since it's already not present
                    reconcile_results.skipped_count += 1;

                    logger::end_action(format!(
                        "Skipped: {}",
                        Paint::yellow("A dependency failed to produce outputs.")
                    ));
                    return;
                }

                let dependency_outputs =
                    self.current_graph.get_dependency_outputs(resource).unwrap();
                let dependencies_hash =
                    ResourceGraph::<TResource, TInputs, TOutputs>::get_dependency_outputs_hash(
                        dependency_outputs,
                    );

                logger::log("Dependencies:");
                logger::log_changeset(get_changeset(
                    &previous_dependencies_hash,
                    &dependencies_hash,
                ));
                logger::log("Inputs:");
                logger::log_changeset(get_changeset(&previous_hash, &inputs_hash));

                match result {
                    CurrentResourceOperationResult::Succeeded(outputs) => {
                        // Add the next resource with its outputs to the next graph
                        let mut next_resource = resource.clone();
                        next_resource.set_outputs(outputs.to_owned());
                        reconcile_results.next_graph.insert_resource(next_resource);
                        reconcile_results.updated_count += 1;

                        logger::end_action_with_results(
                            "Succeeded with outputs:",
                            get_changeset(
                                &previous_resource.get_outputs_hash(),
                                &resource.get_outputs_hash(),
                            ),
                        );
                    }
                    CurrentResourceOperationResult::Failed(reason) => {
                        // Add the previous resource to the next graph because it has not changed
                        reconcile_results
                            .next_graph
                            .insert_resource(previous_resource.clone());
                        reconcile_results.failed_count += 1;

                        logger::end_action(format!("Failed: {}", Paint::red(reason)));
                    }
                    CurrentResourceOperationResult::Skipped(reason) => {
                        // Add the previous resource to the next graph because it has not changed
                        reconcile_results
                            .next_graph
                            .insert_resource(previous_resource.clone());
                        reconcile_results.skipped_count += 1;

                        logger::end_action(format!("Skipped: {}", Paint::yellow(reason)));
                    }
                    _ => unreachable!(),
                }
            }
            CurrentResourceOperation::Recreate { resource_id, .. } => {
                let resource = self.current_graph.get_resource(resource_id).unwrap();
                let inputs_hash = resource.get_inputs_hash();

                let previous_resource = self.previous_graph.get_resource(resource_id).unwrap();
                let previous_hash = previous_resource.get_inputs_hash();
                let previous_dependency_outputs = self
                    .previous_graph
                    .get_dependency_outputs(previous_resource)
                    .unwrap();
                let previous_dependencies_hash =
                    ResourceGraph::<TResource, TInputs, TOutputs>::get_dependency_outputs_hash(
                        previous_dependency_outputs,
                    );

                logger::start_action(format!(
                    "{}/{} Recreating: {}",
                    Paint::red("-"),
                    Paint::green("+"),
                    resource_id
                ));

                if matches!(result, CurrentResourceOperationResult::MissingDependencies) {
                    // No need to update the graph since it's already not present
                    reconcile_results.skipped_count += 1;

                    logger::end_action(format!(
                        "Skipped: {}",
                        Paint::yellow("A dependency failed to produce outputs.")
                    ));
                    return;
                }

                let dependency_outputs =
                    self.current_graph.get_dependency_outputs(resource).unwrap();
                let dependencies_hash =
                    ResourceGraph::<TResource, TInputs, TOutputs>::get_dependency_outputs_hash(
                        dependency_outputs,
                    );

                logger::log("Dependencies:");
                logger::log_changeset(get_changeset(
                    &previous_dependencies_hash,
                    &dependencies_hash,
                ));
                logger::log("Inputs:");
                logger::log_changeset(get_changeset(&previous_hash, &inputs_hash));

                match result {
                    // TODO: Handle delete only!
                    CurrentResourceOperationResult::Succeeded(outputs) => {
                        // Add the next resource with its outputs to the next graph
                        let mut next_resource = resource.clone();
                        next_resource.set_outputs(outputs.to_owned());
                        reconcile_results.next_graph.insert_resource(next_resource);
                        reconcile_results.deleted_count += 1;
                        reconcile_results.created_count += 1;

                        logger::end_action_with_results(
                            "Succeeded with outputs:",
                            get_changeset(
                                &previous_resource.get_outputs_hash(),
                                &resource.get_outputs_hash(),
                            ),
                        );
                    }
                    CurrentResourceOperationResult::Failed(reason) => {
                        // Add the previous resource to the next graph because it has not changed
                        reconcile_results
                            .next_graph
                            .insert_resource(previous_resource.clone());
                        reconcile_results.failed_count += 1;

                        logger::end_action(format!("Failed: {}", Paint::red(reason)));
                    }
                    CurrentResourceOperationResult::Skipped(reason) => {
                        // Add the previous resource to the next graph because it has not changed
                        reconcile_results
                            .next_graph
                            .insert_resource(previous_resource.clone());
                        reconcile_results.skipped_count += 1;

                        logger::end_action(format!("Skipped: {}", Paint::yellow(reason)));
                    }
                    _ => unreachable!(),
                }
            }
            // Ugh.
            CurrentResourceOperation::UpdateOrNoop { resource_id } => todo!(),
            CurrentResourceOperation::Noop => {}
        }
    }

    pub async fn reconcile(
        &self,
    ) -> Result<ReconcileResults<TResource, TInputs, TOutputs>, String> {
        let mut reconcile_results = ReconcileResults {
            created_count: 0,
            updated_count: 0,
            deleted_count: 0,
            noop_count: 0,
            skipped_count: 0,
            failed_count: 0,
            next_graph: ResourceGraph::empty(),
        };

        // Iterate over previous resources in reverse order so that leaf resources are removed first
        let mut previous_resource_order = self.previous_graph.get_topological_order()?;
        previous_resource_order.reverse();
        for resource_id in previous_resource_order {
            let operation = self.get_previous_resource_operation(resource_id);
            let operation_result = self.evaluate_previous_resource_operation(&operation).await;
            self.handle_previous_operation_result(
                &operation,
                &operation_result,
                &mut reconcile_results,
            );
        }

        let mut operations_map = HashMap::new();
        let current_resource_order = self.current_graph.get_topological_order()?;
        for resource_id in current_resource_order {
            let operation =
                self.get_current_resource_operation(resource_id.clone(), &operations_map);
            operations_map.insert(resource_id, operation.clone());

            let operation_result = self.evaluate_current_resource_operation(&operation).await;
            self.handle_current_operation_result(
                &operation,
                &operation_result,
                &mut reconcile_results,
            );
        }

        Ok(reconcile_results)
    }
}
