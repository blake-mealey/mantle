use difference::Changeset;
use serde::Serialize;
use yansi::Paint;

use crate::{resource::Resource, resource_graph::ResourceGraph, resource_manager::ResourceManager};

#[derive(Default, Clone)]
pub struct EvaluateResults {
    pub created_count: u32,
    pub updated_count: u32,
    pub deleted_count: u32,
    pub noop_count: u32,
    pub skipped_count: u32,
}

enum OperationResult<TOutputs> {
    Skipped(String),
    Noop,
    Failed(String),
    SucceededDelete,
    SucceededCreate(TOutputs),
    SucceededUpdate(TOutputs),
}

pub struct ResourceGraphEvaluatorOptions {
    pub allow_purchases: bool,
}

pub struct ResourceGraphEvaluator<'a, TResource, TInputs, TOutputs, TManager>
where
    TResource: Resource<TInputs, TOutputs>,
    TInputs: Clone,
    TOutputs: Clone,
    TOutputs: Serialize,
    TManager: ResourceManager<TInputs, TOutputs>,
{
    previous_graph: &'a ResourceGraph<TResource, TInputs, TOutputs>,
    current_graph: &'a mut ResourceGraph<TResource, TInputs, TOutputs>,
    manager: &'a TManager,
    options: ResourceGraphEvaluatorOptions,
}

impl<'a, TResource, TInputs, TOutputs, TManager>
    ResourceGraphEvaluator<'a, TResource, TInputs, TOutputs, TManager>
where
    TResource: Resource<TInputs, TOutputs>,
    TInputs: Clone,
    TOutputs: Clone,
    TOutputs: Serialize,
    TManager: ResourceManager<TInputs, TOutputs>,
{
    pub fn new(
        previous_graph: &'a ResourceGraph<TResource, TInputs, TOutputs>,
        current_graph: &'a mut ResourceGraph<TResource, TInputs, TOutputs>,
        manager: &'a TManager,
        options: ResourceGraphEvaluatorOptions,
    ) -> Self {
        Self {
            previous_graph,
            current_graph,
            manager,
            options,
        }
    }

    pub async fn evaluate(&mut self) -> Result<EvaluateResults, String>
    where
        TManager: ResourceManager<TInputs, TOutputs>,
    {
        let mut results = EvaluateResults::default();
        let mut failures_count: u32 = 0;

        // Iterate over previous resources in reverse order so that leaf resources are removed first
        let mut previous_resource_order = self.previous_graph.get_topological_order()?;
        previous_resource_order.reverse();
        for resource_id in previous_resource_order.iter() {
            if self.current_graph.resources.get(resource_id).is_some() {
                continue;
            }

            let operation_result = self.evaluate_delete(resource_id).await;
            self.handle_operation_result(
                &mut results,
                &mut failures_count,
                resource_id,
                operation_result,
            );
        }

        let resource_order = self.current_graph.get_topological_order()?;
        for resource_id in resource_order.iter() {
            let operation_result = self.evaluate_create_or_update(resource_id).await;
            self.handle_operation_result(
                &mut results,
                &mut failures_count,
                resource_id,
                operation_result,
            );
        }

        if failures_count > 0 {
            Err(format!(
                "Failed {} changes(s) while evaluating the resource graph. See above for more details.",
                failures_count
            ))
        } else {
            Ok(results)
        }
    }

    fn handle_operation_result(
        &mut self,
        results: &mut EvaluateResults,
        failures_count: &mut u32,
        resource_id: &str,
        operation_result: OperationResult<TOutputs>,
    ) {
        // TODO: Improve DRY here
        match operation_result {
            OperationResult::SucceededDelete => {
                // No need to update the graph since it's already not present
                results.deleted_count += 1;
                let previous_resource = self.previous_graph.resources.get(resource_id).unwrap();
                logger::end_action_with_results(
                    "Succeeded with outputs:",
                    get_changeset(&previous_resource.get_outputs_hash(), ""),
                );
            }
            OperationResult::SucceededCreate(outputs) => {
                // Update the resource with the new outputs
                let resource = self.current_graph.resources.get_mut(resource_id).unwrap();
                resource.set_outputs(outputs);

                results.created_count += 1;
                logger::end_action_with_results(
                    "Succeeded with outputs:",
                    get_changeset("", &resource.get_outputs_hash()),
                );
            }
            OperationResult::SucceededUpdate(outputs) => {
                // Update the resource with the new outputs
                let resource = self.current_graph.resources.get_mut(resource_id).unwrap();
                resource.set_outputs(outputs);

                results.updated_count += 1;
                let previous_resource = self.previous_graph.resources.get(resource_id).unwrap();
                logger::end_action_with_results(
                    "Succeeded with outputs:",
                    get_changeset(
                        &previous_resource.get_outputs_hash(),
                        &resource.get_outputs_hash(),
                    ),
                );
            }
            OperationResult::Noop => {
                // There was no need to create or update the resource. We will update the resource
                // with the previous outputs
                let previous_resource = self.previous_graph.resources.get(resource_id).unwrap();
                let resource = self.current_graph.resources.get_mut(resource_id).unwrap();
                resource.set_outputs(
                    previous_resource
                        .get_outputs()
                        .expect("Existing resource should have outputs."),
                );

                results.noop_count += 1;
            }
            OperationResult::Skipped(reason) => {
                // The resource was not evaluated. If the resource existed previously, we will copy
                // the old version into this graph. Otherwise, we will remove this resource from the
                // graph.
                if let Some(previous_resource) = self.previous_graph.resources.get(resource_id) {
                    self.current_graph
                        .resources
                        .insert(resource_id.to_owned(), previous_resource.to_owned());
                } else {
                    self.current_graph.resources.remove(resource_id);
                }

                results.skipped_count += 1;
                logger::end_action(format!("Skipped: {}", Paint::yellow(reason)));
            }
            OperationResult::Failed(error) => {
                // An error occurred while creating or updating the resource. If the
                // resource existed previously, we will copy the old version into this
                // graph. Otherwise, we will remove this resource from the graph.
                if let Some(previous_resource) = self.previous_graph.resources.get(resource_id) {
                    self.current_graph
                        .resources
                        .insert(resource_id.to_owned(), previous_resource.to_owned());
                } else {
                    self.current_graph.resources.remove(resource_id);
                }

                *failures_count += 1;
                logger::end_action(format!("Failed: {}", Paint::red(error)));
            }
        }
    }

    async fn evaluate_delete(&self, resource_id: &str) -> OperationResult<TOutputs>
    where
        TManager: ResourceManager<TInputs, TOutputs>,
    {
        let resource = self.previous_graph.resources.get(resource_id).unwrap();
        let dependency_outputs = self
            .previous_graph
            .get_dependency_outputs(resource)
            .expect("Previous graph should be complete.");

        let inputs_hash = resource.get_inputs_hash();
        let dependencies_hash = self
            .current_graph
            .get_dependency_outputs_hash(dependency_outputs.clone());
        logger::start_action(format!(
            "{} Deleting: {}",
            Paint::red("-"),
            resource.get_id()
        ));
        logger::log("Dependencies:");
        logger::log_changeset(get_changeset(&dependencies_hash, &dependencies_hash));
        logger::log("Inputs:");
        logger::log_changeset(get_changeset(&inputs_hash, ""));

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
            Ok(()) => OperationResult::SucceededDelete,
            Err(error) => OperationResult::Failed(error),
        }
    }

    async fn evaluate_create_or_update(&self, resource_id: &str) -> OperationResult<TOutputs>
    where
        TManager: ResourceManager<TInputs, TOutputs>,
    {
        let resource = self.current_graph.resources.get(resource_id).unwrap();
        let inputs_hash = resource.get_inputs_hash();
        let dependency_outputs = self.current_graph.get_dependency_outputs(resource);

        let previous_resource = self.previous_graph.resources.get(resource_id);

        if let Some(previous_resource) = previous_resource {
            // Check for changes
            let previous_hash = previous_resource.get_inputs_hash();
            let previous_dependency_outputs = self
                .previous_graph
                .get_dependency_outputs(previous_resource)
                .expect("Previous graph should be complete.");
            let previous_dependencies_hash = self
                .current_graph
                .get_dependency_outputs_hash(previous_dependency_outputs);

            // TODO: How can we determine between update/noop?
            let dependency_outputs = match dependency_outputs {
                Some(v) => v,
                None => {
                    logger::start_action(format!(
                        "{} Update or Noop: {}",
                        Paint::new("○").dimmed(),
                        resource.get_id(),
                    ));
                    return OperationResult::Skipped(
                        "A dependency failed to produce outputs.".to_owned(),
                    );
                }
            };
            let dependencies_hash = self
                .current_graph
                .get_dependency_outputs_hash(dependency_outputs.clone());

            if previous_hash == inputs_hash && previous_dependencies_hash == dependencies_hash {
                // No changes
                return OperationResult::Noop;
            }

            // This resource has changed
            logger::start_action(format!("{} Updating: {}", Paint::yellow("~"), resource_id));
            logger::log("Dependencies:");
            logger::log_changeset(get_changeset(
                &previous_dependencies_hash,
                &dependencies_hash,
            ));
            logger::log("Inputs:");
            logger::log_changeset(get_changeset(&previous_hash, &inputs_hash));

            let outputs = previous_resource
                .get_outputs()
                .expect("Existing resource should have outputs.");

            let price = match self
                .manager
                .get_update_price(
                    resource.get_inputs(),
                    outputs.clone(),
                    dependency_outputs.clone(),
                )
                .await
            {
                Ok(Some(price)) if price > 0 => {
                    if self.options.allow_purchases {
                        logger::log("");
                        logger::log(Paint::yellow(format!(
                            "{} Robux will be charged from your account.",
                            price
                        )));
                        Some(price)
                    } else {
                        return OperationResult::Skipped(format!(
                                "Resource would cost {} Robux to create. Give Mantle permission to make purchases with --allow-purchases.",
                                price
                            ));
                    }
                }
                Err(error) => return OperationResult::Failed(error),
                Ok(_) => None,
            };

            match self
                .manager
                .update(resource.get_inputs(), outputs, dependency_outputs, price)
                .await
            {
                Ok(outputs) => OperationResult::SucceededUpdate(outputs),
                Err(error) => OperationResult::Failed(error),
            }
        } else {
            // Create
            logger::start_action(format!("{} Creating: {}", Paint::green("+"), resource_id));

            let dependency_outputs = match dependency_outputs {
                Some(v) => v,
                None => {
                    return OperationResult::Skipped(
                        "A dependency failed to produce outputs.".to_owned(),
                    );
                }
            };
            let dependencies_hash = self
                .current_graph
                .get_dependency_outputs_hash(dependency_outputs.clone());

            logger::log("Dependencies:");
            logger::log_changeset(get_changeset(&dependencies_hash, &dependencies_hash));
            logger::log("Inputs:");
            logger::log_changeset(get_changeset("", &inputs_hash));

            let price = match self
                .manager
                .get_create_price(resource.get_inputs(), dependency_outputs.clone())
                .await
            {
                Ok(Some(price)) if price > 0 => {
                    if self.options.allow_purchases {
                        logger::log("");
                        logger::log(Paint::yellow(format!(
                            "{} Robux will be charged from your account.",
                            price
                        )));
                        Some(price)
                    } else {
                        return OperationResult::Skipped(format!(
                                "Resource would cost {} Robux to create. Give Mantle permission to make purchases with --allow-purchases.",
                                price
                            ));
                    }
                }
                Err(error) => return OperationResult::Failed(error),
                Ok(_) => None,
            };

            match self
                .manager
                .create(resource.get_inputs(), dependency_outputs, price)
                .await
            {
                Ok(outputs) => OperationResult::SucceededCreate(outputs),
                Err(error) => OperationResult::Failed(error),
            }
        }
    }
}

fn get_changeset(previous_hash: &str, new_hash: &str) -> Changeset {
    Changeset::new(previous_hash, new_hash, "\n")
}
