use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

use async_trait::async_trait;
use difference::Changeset;
use serde::Serialize;
use yansi::Paint;

use crate::logger;

macro_rules! all_outputs {
    ($expr:expr, $enum:path) => {{
        $expr
            .iter()
            .filter_map(|value| {
                if let $enum(outputs) = value {
                    Some(outputs)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }};
}
pub(crate) use all_outputs;

macro_rules! single_output {
    ($expr:expr, $enum:path) => {{
        *all_outputs!($expr, $enum)
            .first()
            .expect("Missing expected output")
    }};
}
pub(crate) use single_output;

macro_rules! optional_output {
    ($expr:expr, $enum:path) => {{
        all_outputs!($expr, $enum).first().map(|output| *output)
    }};
}
pub(crate) use optional_output;

pub type ResourceId = String;

pub trait Resource<TInputs, TOutputs>: Clone {
    fn get_id(&self) -> ResourceId;
    fn get_inputs_hash(&self) -> String;
    fn get_outputs_hash(&self) -> String;
    fn get_inputs(&self) -> TInputs;
    fn get_outputs(&self) -> Option<TOutputs>;
    fn get_dependencies(&self) -> Vec<ResourceId>;
    fn set_outputs(&mut self, outputs: TOutputs);
}

#[async_trait]
pub trait ResourceManager<TInputs, TOutputs> {
    async fn get_create_price(
        &self,
        inputs: TInputs,
        dependency_outputs: Vec<TOutputs>,
    ) -> Result<Option<u32>, String>;

    async fn create(
        &self,
        inputs: TInputs,
        dependency_outputs: Vec<TOutputs>,
    ) -> Result<TOutputs, String>;

    async fn get_update_price(
        &self,
        inputs: TInputs,
        outputs: TOutputs,
        dependency_outputs: Vec<TOutputs>,
    ) -> Result<Option<u32>, String>;

    async fn update(
        &self,
        inputs: TInputs,
        outputs: TOutputs,
        dependency_outputs: Vec<TOutputs>,
    ) -> Result<TOutputs, String>;

    async fn delete(
        &self,
        outputs: TOutputs,
        dependency_outputs: Vec<TOutputs>,
    ) -> Result<(), String>;
}

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

fn get_changeset(previous_hash: &str, new_hash: &str) -> Changeset {
    Changeset::new(previous_hash, new_hash, "\n")
}

pub struct ResourceGraph<TResource, TInputs, TOutputs>
where
    TResource: Resource<TInputs, TOutputs>,
    TInputs: Clone,
    TOutputs: Clone,
{
    phantom_inputs: std::marker::PhantomData<TInputs>,
    phantom_outputs: std::marker::PhantomData<TOutputs>,
    resources: HashMap<ResourceId, TResource>,
}

impl<TResource, TInputs, TOutputs> ResourceGraph<TResource, TInputs, TOutputs>
where
    TResource: Resource<TInputs, TOutputs>,
    TInputs: Clone,
    TOutputs: Clone,
    TOutputs: Serialize,
{
    pub fn new(resources: &[TResource]) -> Self {
        Self {
            resources: resources
                .iter()
                .map(|resource| (resource.get_id(), resource.clone()))
                .collect(),
            phantom_inputs: PhantomData,
            phantom_outputs: PhantomData,
        }
    }

    pub fn get_outputs(&self, resource_id: &str) -> Option<TOutputs> {
        self.resources
            .get(resource_id)
            .map(|resource| resource.get_outputs())
            .flatten()
    }

    fn get_dependency_graph(&self) -> BTreeMap<ResourceId, Vec<ResourceId>> {
        self.resources
            .iter()
            .map(|(id, resource)| (id.clone(), resource.get_dependencies()))
            .collect()
    }

    fn get_topological_order(&self) -> Result<Vec<ResourceId>, String> {
        let mut dependency_graph = self.get_dependency_graph();

        let mut start_nodes: Vec<ResourceId> = dependency_graph
            .iter()
            .filter_map(|(node, deps)| {
                if deps.is_empty() {
                    Some(node.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut ordered: Vec<ResourceId> = Vec::new();
        while let Some(start_node) = start_nodes.pop() {
            ordered.push(start_node.clone());
            for (node, deps) in dependency_graph.iter_mut() {
                if deps.contains(&start_node) {
                    deps.retain(|dep| dep != &start_node);
                    if deps.is_empty() {
                        start_nodes.push(node.clone());
                    }
                }
            }
        }

        let has_cycles = dependency_graph.iter().any(|(_, deps)| !deps.is_empty());
        match has_cycles {
            true => Err("Cannot evaluate resource graph because it has cycles".to_owned()),
            false => Ok(ordered),
        }
    }

    pub fn get_resource_list(&self) -> Vec<TResource> {
        self.get_topological_order()
            .unwrap()
            .iter()
            .map(|id| self.resources.get(id).unwrap().clone())
            .collect()
    }

    fn get_dependency_outputs(&self, resource: &TResource) -> Option<Vec<TOutputs>> {
        let mut dependency_outputs: Vec<TOutputs> = Vec::new();
        for dependency in resource.get_dependencies() {
            let resource = self.resources.get(&dependency);
            if let Some(resource) = resource {
                if let Some(outputs) = resource.get_outputs() {
                    dependency_outputs.push(outputs);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        Some(dependency_outputs)
    }

    fn get_dependency_outputs_hash(&self, dependency_outputs: Vec<TOutputs>) -> String {
        // TODO: Should we separate hashes from displays?
        let hash = serde_yaml::to_string(&dependency_outputs)
            .map_err(|e| format!("Failed to compute dependency outputs hash\n\t{}", e))
            .unwrap();
        if hash.is_empty() {
            ""
        } else {
            // We remove first 4 characters to remove "---\n", and we trim the end to remove "\n"
            hash[4..].trim_end()
        }
        .to_owned()
    }

    fn handle_operation_result(
        &mut self,
        results: &mut EvaluateResults,
        failures_count: &mut u32,
        previous_graph: &ResourceGraph<TResource, TInputs, TOutputs>,
        resource_id: &str,
        operation_result: OperationResult<TOutputs>,
    ) {
        // TODO: Improve DRY here
        match operation_result {
            OperationResult::SucceededDelete => {
                // No need to update the graph since it's already not present
                results.deleted_count += 1;
                let previous_resource = previous_graph.resources.get(resource_id).unwrap();
                logger::end_action_with_results(
                    "Succeeded with outputs:",
                    get_changeset(&previous_resource.get_outputs_hash(), ""),
                );
            }
            OperationResult::SucceededCreate(outputs) => {
                // Update the resource with the new outputs
                let resource = self.resources.get_mut(resource_id).unwrap();
                resource.set_outputs(outputs);

                results.created_count += 1;
                logger::end_action_with_results(
                    "Succeeded with outputs:",
                    get_changeset("", &resource.get_outputs_hash()),
                );
            }
            OperationResult::SucceededUpdate(outputs) => {
                // Update the resource with the new outputs
                let resource = self.resources.get_mut(resource_id).unwrap();
                resource.set_outputs(outputs);

                results.updated_count += 1;
                let previous_resource = previous_graph.resources.get(resource_id).unwrap();
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
                let previous_resource = previous_graph.resources.get(resource_id).unwrap();
                let resource = self.resources.get_mut(resource_id).unwrap();
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
                if let Some(previous_resource) = previous_graph.resources.get(resource_id) {
                    self.resources
                        .insert(resource_id.to_owned(), previous_resource.to_owned());
                } else {
                    self.resources.remove(resource_id);
                }

                results.skipped_count += 1;
                logger::end_action(format!("Skipped: {}", Paint::yellow(reason)));
            }
            OperationResult::Failed(error) => {
                // An error occurred while creating or updating the resource. If the
                // resource existed previously, we will copy the old version into this
                // graph. Otherwise, we will remove this resource from the graph.
                if let Some(previous_resource) = previous_graph.resources.get(resource_id) {
                    self.resources
                        .insert(resource_id.to_owned(), previous_resource.to_owned());
                } else {
                    self.resources.remove(resource_id);
                }

                *failures_count += 1;
                logger::end_action(format!("Failed: {}", Paint::red(error)));
            }
        }
    }

    async fn evaluate_delete<TManager>(
        &self,
        previous_graph: &ResourceGraph<TResource, TInputs, TOutputs>,
        manager: &mut TManager,
        resource_id: &str,
    ) -> OperationResult<TOutputs>
    where
        TManager: ResourceManager<TInputs, TOutputs>,
    {
        let resource = previous_graph.resources.get(resource_id).unwrap();
        let dependency_outputs = previous_graph
            .get_dependency_outputs(resource)
            .expect("Previous graph should be complete.");

        let inputs_hash = resource.get_inputs_hash();
        let dependencies_hash = self.get_dependency_outputs_hash(dependency_outputs.clone());
        logger::start_action(format!(
            "{} Deleting: {}",
            Paint::red("-"),
            resource.get_id()
        ));
        logger::log("Dependencies:");
        logger::log_changeset(get_changeset(&dependencies_hash, &dependencies_hash));
        logger::log("Inputs:");
        logger::log_changeset(get_changeset(&inputs_hash, ""));

        match manager
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

    async fn evaluate_create_or_update<TManager>(
        &self,
        previous_graph: &ResourceGraph<TResource, TInputs, TOutputs>,
        manager: &mut TManager,
        resource_id: &str,
        allow_purchases: bool,
    ) -> OperationResult<TOutputs>
    where
        TManager: ResourceManager<TInputs, TOutputs>,
    {
        let resource = self.resources.get(resource_id).unwrap();
        let inputs_hash = resource.get_inputs_hash();
        let dependency_outputs = self.get_dependency_outputs(resource);

        let previous_resource = previous_graph.resources.get(resource_id);

        if let Some(previous_resource) = previous_resource {
            // Check for changes
            let previous_hash = previous_resource.get_inputs_hash();
            let previous_dependency_outputs = previous_graph
                .get_dependency_outputs(previous_resource)
                .expect("Previous graph should be complete.");
            let previous_dependencies_hash =
                self.get_dependency_outputs_hash(previous_dependency_outputs);

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
            let dependencies_hash = self.get_dependency_outputs_hash(dependency_outputs.clone());

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

            match manager
                .get_update_price(
                    resource.get_inputs(),
                    outputs.clone(),
                    dependency_outputs.clone(),
                )
                .await
            {
                Ok(Some(price)) if price > 0 => {
                    if allow_purchases {
                        logger::log("");
                        logger::log(Paint::yellow(format!(
                            "{} Robux will be charged from your account.",
                            price
                        )))
                    } else {
                        return OperationResult::Skipped(format!(
                                "Resource would cost {} Robux to create. Give Mantle permission to make purchases with --allow-purchases.",
                                price
                            ));
                    }
                }
                Err(error) => return OperationResult::Failed(error),
                Ok(_) => {}
            };

            match manager
                .update(resource.get_inputs(), outputs, dependency_outputs)
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
            let dependencies_hash = self.get_dependency_outputs_hash(dependency_outputs.clone());

            logger::log("Dependencies:");
            logger::log_changeset(get_changeset(&dependencies_hash, &dependencies_hash));
            logger::log("Inputs:");
            logger::log_changeset(get_changeset("", &inputs_hash));

            match manager
                .get_create_price(resource.get_inputs(), dependency_outputs.clone())
                .await
            {
                Ok(Some(price)) if price > 0 => {
                    if allow_purchases {
                        logger::log("");
                        logger::log(Paint::yellow(format!(
                            "{} Robux will be charged from your account.",
                            price
                        )))
                    } else {
                        return OperationResult::Skipped(format!(
                                "Resource would cost {} Robux to create. Give Mantle permission to make purchases with --allow-purchases.",
                                price
                            ));
                    }
                }
                Err(error) => return OperationResult::Failed(error),
                Ok(_) => {}
            };

            match manager
                .create(resource.get_inputs(), dependency_outputs)
                .await
            {
                Ok(outputs) => OperationResult::SucceededCreate(outputs),
                Err(error) => OperationResult::Failed(error),
            }
        }
    }

    pub async fn evaluate<TManager>(
        &mut self,
        previous_graph: &ResourceGraph<TResource, TInputs, TOutputs>,
        manager: &mut TManager,
        allow_purchases: bool,
    ) -> Result<EvaluateResults, String>
    where
        TManager: ResourceManager<TInputs, TOutputs>,
    {
        let mut results = EvaluateResults::default();
        let mut failures_count: u32 = 0;

        // Iterate over previous resources in reverse order so that leaf resources are removed first
        let mut previous_resource_order = previous_graph.get_topological_order()?;
        previous_resource_order.reverse();
        for resource_id in previous_resource_order.iter() {
            if self.resources.get(resource_id).is_some() {
                continue;
            }

            let operation_result = self
                .evaluate_delete(previous_graph, manager, resource_id)
                .await;
            self.handle_operation_result(
                &mut results,
                &mut failures_count,
                previous_graph,
                resource_id,
                operation_result,
            );
        }

        let resource_order = self.get_topological_order()?;
        for resource_id in resource_order.iter() {
            let operation_result = self
                .evaluate_create_or_update(previous_graph, manager, resource_id, allow_purchases)
                .await;
            self.handle_operation_result(
                &mut results,
                &mut failures_count,
                previous_graph,
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
}
