use std::collections::{BTreeMap, HashMap};

use difference::Changeset;
use serde::{Deserialize, Serialize};
use yansi::Paint;

use crate::logger;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub resource_type: String,
    pub id: String,
    pub inputs: BTreeMap<String, Input>,
    pub outputs: Option<BTreeMap<String, OutputValue>>,
}

pub struct ResourceDiff {
    pub previous_hash: Option<String>,
    pub previous_resource: Option<Resource>,
    pub resource: Resource,
}

impl Resource {
    pub fn new(resource_type: &str, id: &str) -> Self {
        Resource {
            resource_type: resource_type.to_owned(),
            id: id.to_owned(),
            inputs: BTreeMap::new(),
            outputs: None,
        }
    }

    pub fn add_value_input<T>(&mut self, name: &str, input_value: &T) -> Result<&mut Self, String>
    where
        T: serde::Serialize,
    {
        let serialized_value = serde_yaml::to_value(input_value)
            .map_err(|e| format!("Failed to serialize input value:\n\t{}", e))?;
        self.inputs
            .insert(name.to_owned(), Input::Value(serialized_value));
        Ok(self)
    }

    pub fn add_ref_input(&mut self, name: &str, input_ref: &InputRef) -> &mut Self {
        self.inputs
            .insert(name.to_owned(), Input::Ref(input_ref.clone()));
        self
    }

    pub fn add_ref_input_list(&mut self, name: &str, input_ref_list: &[InputRef]) -> &mut Self {
        self.inputs
            .insert(name.to_owned(), Input::RefList(input_ref_list.to_owned()));
        self
    }

    pub fn get_ref(&self) -> ResourceRef {
        (self.resource_type.clone(), self.id.clone())
    }

    pub fn get_input_ref(&self, input_ref_output: &str) -> InputRef {
        (
            self.resource_type.clone(),
            self.id.clone(),
            input_ref_output.to_owned(),
        )
    }

    pub fn get_output_from_input_ref(&self, input_ref: &InputRef) -> Result<OutputValue, String> {
        self.get_output(&output_name_from_input_ref(input_ref))
    }

    pub fn get_output(&self, name: &str) -> Result<OutputValue, String> {
        if let Some(outputs) = &self.outputs {
            let value = outputs
                .get(name)
                .ok_or(format!("No output with name: {:?}", name))?;
            Ok(value.clone())
        } else {
            return Err(format!(
                "Resource {}.{} has no outputs",
                self.resource_type, self.id
            ));
        }
    }

    fn get_dependency_refs(&self) -> Vec<ResourceRef> {
        self.inputs
            .values()
            .filter_map(|input| match input {
                Input::Ref(ref input_ref) => Some(vec![input_ref]),
                Input::RefList(ref input_ref_list) => Some(input_ref_list.iter().collect()),
                _ => None,
            })
            .flatten()
            .map(resource_ref_from_input_ref)
            .collect()
    }
}

pub type InputRef = (String, String, String);
pub type InputValue = serde_yaml::Value;
pub type OutputValue = serde_yaml::Value;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Input {
    Ref(InputRef),
    RefList(Vec<InputRef>),
    Value(InputValue),
}

pub type ResourceRef = (String, String);

pub fn resource_ref_from_input_ref(input_ref: &InputRef) -> ResourceRef {
    let (input_ref_type, input_ref_id, _) = input_ref;
    (input_ref_type.clone(), input_ref_id.clone())
}

pub fn output_name_from_input_ref(input_ref: &InputRef) -> String {
    let (_, _, input_ref_output) = input_ref;
    input_ref_output.clone()
}
pub trait ResourceManager {
    fn get_create_price(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
    ) -> Result<Option<u32>, String>;

    fn create(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
    ) -> Result<Option<serde_yaml::Value>, String>;

    fn update(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
        resource_outputs: serde_yaml::Value,
    ) -> Result<Option<serde_yaml::Value>, String>;

    fn delete(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
        resource_outputs: serde_yaml::Value,
    ) -> Result<(), String>;
}

fn format_inputs_hash(inputs_hash: &str) -> &str {
    // We remove first 4 characters to remove "---\n", and we trim the end to remove "\n"
    if inputs_hash.is_empty() {
        ""
    } else {
        inputs_hash[4..].trim_end()
    }
}

fn get_changeset(previous_inputs_hash: &str, new_inputs_hash: &str) -> Changeset {
    Changeset::new(
        format_inputs_hash(previous_inputs_hash),
        format_inputs_hash(new_inputs_hash),
        "\n",
    )
}

#[derive(Default, Clone)]
pub struct EvaluateResults {
    pub created_count: u32,
    pub updated_count: u32,
    pub deleted_count: u32,
    pub noop_count: u32,
    pub skipped_count: u32,
}

enum OperationType {
    Create,
    Update,
}

enum OperationResult {
    Skipped(String),
    Noop,
    Failed(String),
    Succeeded(OperationType, Option<BTreeMap<String, OutputValue>>),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceGraph {
    pub resources: HashMap<ResourceRef, Resource>,
}

impl ResourceGraph {
    pub fn new(resources: &[Resource]) -> Self {
        let mut graph = ResourceGraph {
            resources: HashMap::new(),
        };
        for resource in resources {
            graph = graph.add_resource(resource);
        }
        graph
    }

    pub fn add_resource(mut self, resource: &Resource) -> Self {
        self.resources.insert(resource.get_ref(), resource.clone());
        self
    }

    pub fn get_resource_list(&self) -> Vec<Resource> {
        let mut resources: Vec<Resource> = self.resources.values().cloned().collect();
        resources.sort_by_key(|a| a.get_ref());
        resources
    }

    pub fn get_resource_from_ref(&self, resource_ref: &ResourceRef) -> Option<Resource> {
        self.resources.get(resource_ref).cloned()
    }

    pub fn get_resource_from_input_ref(&self, input_ref: &InputRef) -> Option<Resource> {
        self.get_resource_from_ref(&resource_ref_from_input_ref(input_ref))
    }

    fn resolve_inputs(
        &self,
        resource: &Resource,
    ) -> Result<Option<BTreeMap<String, InputValue>>, String> {
        let mut resolved_inputs: BTreeMap<String, InputValue> = BTreeMap::new();
        for (name, input) in &resource.inputs {
            match input {
                Input::Value(value) => {
                    resolved_inputs.insert(name.clone(), value.clone());
                }
                Input::Ref(value_ref) => {
                    let referenced_resource = self.get_resource_from_input_ref(value_ref);
                    let output = match referenced_resource {
                        None => None,
                        Some(Resource {
                            outputs: None,
                            resource_type: _,
                            id: _,
                            inputs: _,
                        }) => None,
                        Some(resource) => Some(resource.get_output_from_input_ref(value_ref)?),
                    };
                    if let Some(output) = output {
                        resolved_inputs.insert(name.clone(), output);
                    } else {
                        return Ok(None);
                    }
                }
                Input::RefList(ref_list) => {
                    let mut resolved_values = Vec::new();
                    for value_ref in ref_list {
                        let referenced_resource = self.get_resource_from_input_ref(value_ref);
                        let output = match referenced_resource {
                            None => None,
                            Some(Resource {
                                outputs: None,
                                resource_type: _,
                                id: _,
                                inputs: _,
                            }) => None,
                            Some(resource) => Some(resource.get_output_from_input_ref(value_ref)?),
                        };
                        if let Some(output) = output {
                            resolved_values.push(output);
                        } else {
                            return Ok(None);
                        }
                    }
                    resolved_inputs.insert(
                        name.clone(),
                        serde_yaml::to_value(resolved_values).map_err(|e| {
                            format!("Failed to serialize resolved ref list\n\t{}", e)
                        })?,
                    );
                }
            }
        }
        Ok(Some(resolved_inputs))
    }

    pub fn get_inputs_hash(&self, inputs: &BTreeMap<String, InputValue>) -> Result<String, String> {
        // TODO: should we actually hash this to make comparisons snappier?
        serde_yaml::to_string(&inputs).map_err(|e| format!("Failed to compute input hash\n\t{}", e))
    }

    fn get_dependency_graph(&self) -> HashMap<ResourceRef, Vec<ResourceRef>> {
        let mut dependency_graph = HashMap::new();
        for resource in self.resources.values() {
            dependency_graph.insert(resource.get_ref(), resource.get_dependency_refs());
        }
        dependency_graph
    }

    fn get_topological_order(&self) -> Result<Vec<ResourceRef>, String> {
        let mut dependency_graph = self.get_dependency_graph();

        let mut start_nodes: Vec<ResourceRef> = dependency_graph
            .iter()
            .filter_map(|(node, deps)| {
                if deps.is_empty() {
                    Some(node.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut ordered: Vec<ResourceRef> = Vec::new();
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

    fn get_resource_diff(
        &self,
        previous_graph: &ResourceGraph,
        resource_ref: &ResourceRef,
    ) -> Result<ResourceDiff, String> {
        let resource = self.get_resource_from_ref(resource_ref).unwrap();
        match &previous_graph.get_resource_from_ref(resource_ref) {
            Some(previous_resource) => {
                let previous_inputs = previous_graph
                    .resolve_inputs(previous_resource)?
                    .ok_or("Previous graph should be complete.")?;
                Ok(ResourceDiff {
                    previous_hash: Some(previous_graph.get_inputs_hash(&previous_inputs)?),
                    previous_resource: Some(previous_resource.clone()),
                    resource: Resource {
                        id: resource.id.clone(),
                        inputs: resource.inputs.clone(),
                        outputs: previous_resource.outputs.clone(),
                        resource_type: resource.resource_type,
                    },
                })
            }
            None => Ok(ResourceDiff {
                previous_hash: None,
                previous_resource: None,
                resource,
            }),
        }
    }

    fn evaluate_create_or_update<TManager>(
        &self,
        resource_manager: &mut TManager,
        resource_diff: &ResourceDiff,
        allow_purchases: bool,
    ) -> OperationResult
    where
        TManager: ResourceManager,
    {
        let ResourceDiff {
            previous_hash,
            resource,
            ..
        } = resource_diff;

        let resolved_inputs = match self.resolve_inputs(&resource_diff.resource) {
            Ok(v) => v,
            Err(e) => return OperationResult::Failed(e),
        };

        let inputs = match resolved_inputs {
            Some(v) => v,
            None => {
                logger::start_action(format!(
                    "{} Unknown: {} {}",
                    Paint::new("○").dimmed(),
                    resource.resource_type,
                    resource.id
                ));
                return OperationResult::Skipped(
                    "A dependency failed to produce required outputs.".to_owned(),
                );
            }
        };

        let inputs_hash = match self.get_inputs_hash(&inputs) {
            Ok(v) => v,
            Err(e) => return OperationResult::Failed(e),
        };

        // TODO: improve DRY here
        match previous_hash {
            None => {
                // This resource is new
                logger::start_action(format!(
                    "{} Creating: {} {}",
                    Paint::green("+"),
                    resource.resource_type,
                    resource.id
                ));
                logger::log_changeset(get_changeset("", &inputs_hash));

                let inputs = match serde_yaml::to_value(inputs) {
                    Ok(v) => v,
                    Err(e) => {
                        return OperationResult::Failed(format!(
                            "Unable to serialize inputs: {}",
                            e
                        ))
                    }
                };

                match resource_manager.get_create_price(&resource.resource_type, inputs.clone()) {
                    Ok(Some(price)) if price > 0 => {
                        if allow_purchases {
                            logger::log("");
                            logger::log(Paint::yellow(format!(
                                "{} Robux will be charged from your account.",
                                price
                            )))
                        } else {
                            return OperationResult::Skipped(format!(
                                "Resource would cost {} Robux to create. Give Rocat permission to make purchases with --allow-purchases.",
                                price
                            ));
                        }
                    }
                    Err(e) => {
                        return OperationResult::Failed(format!(
                            "Unable to get create price: {}",
                            e
                        ))
                    }
                    Ok(None) => {}
                    Ok(Some(_)) => {}
                };

                match resource_manager.create(&resource.resource_type, inputs) {
                    Ok(Some(outputs)) => {
                        let outputs = match serde_yaml::from_value::<BTreeMap<String, OutputValue>>(
                            outputs,
                        ) {
                            Ok(v) => v,
                            Err(e) => {
                                return OperationResult::Failed(format!(
                                    "Unable to deserialize outputs: {}",
                                    e
                                ))
                            }
                        };
                        OperationResult::Succeeded(OperationType::Create, Some(outputs))
                    }
                    Ok(None) => OperationResult::Succeeded(OperationType::Create, None),
                    Err(error) => OperationResult::Failed(error),
                }
            }
            Some(previous_hash) if *previous_hash != inputs_hash => {
                // This resource has changed
                logger::start_action(format!(
                    "{} Updating: {} {}",
                    Paint::yellow("~"),
                    resource.resource_type,
                    resource.id
                ));
                logger::log_changeset(get_changeset(previous_hash, &inputs_hash));

                let inputs = match serde_yaml::to_value(inputs) {
                    Ok(v) => v,
                    Err(e) => {
                        return OperationResult::Failed(format!(
                            "Unable to serialize inputs: {}",
                            e
                        ))
                    }
                };
                let outputs = resource.outputs.clone().unwrap_or_default();
                let outputs = match serde_yaml::to_value(outputs) {
                    Ok(v) => v,
                    Err(e) => {
                        return OperationResult::Failed(format!(
                            "Unable to serialize outputs: {}",
                            e
                        ))
                    }
                };

                match resource_manager.update(&resource.resource_type, inputs, outputs) {
                    Ok(Some(outputs)) => {
                        let outputs = match serde_yaml::from_value::<BTreeMap<String, OutputValue>>(
                            outputs,
                        ) {
                            Ok(v) => v,
                            Err(e) => {
                                return OperationResult::Failed(format!(
                                    "Unable to deserialize outputs: {}",
                                    e
                                ))
                            }
                        };
                        OperationResult::Succeeded(OperationType::Update, Some(outputs))
                    }
                    Ok(None) => OperationResult::Succeeded(OperationType::Update, None),
                    Err(error) => OperationResult::Failed(error),
                }
            }
            _ => OperationResult::Noop,
        }
    }

    pub fn evaluate<TManager>(
        &mut self,
        previous_graph: &ResourceGraph,
        resource_manager: &mut TManager,
        allow_purchases: bool,
    ) -> Result<EvaluateResults, String>
    where
        TManager: ResourceManager,
    {
        let resource_order = self.get_topological_order()?;

        let mut results: EvaluateResults = Default::default();
        let mut failures_count = 0;

        for resource_ref in resource_order {
            let resource_diff = self.get_resource_diff(previous_graph, &resource_ref)?;

            let operation_result =
                self.evaluate_create_or_update(resource_manager, &resource_diff, allow_purchases);

            match operation_result {
                OperationResult::Succeeded(op_type, outputs) => {
                    let mut resource_with_outputs = resource_diff.resource.clone();
                    resource_with_outputs.outputs = outputs.clone();
                    self.resources.insert(resource_ref, resource_with_outputs);

                    match op_type {
                        OperationType::Create => results.created_count += 1,
                        OperationType::Update => results.updated_count += 1,
                    };

                    match outputs {
                        None => logger::end_action("Succeeded"),
                        Some(outputs) => logger::end_action_with_results(
                            "Succeeded with outputs:",
                            format_inputs_hash(
                                &serde_yaml::to_string(&outputs)
                                    .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                            ),
                        ),
                    };
                }
                OperationResult::Noop => {
                    // There was no need to create or update the resource. We will update the graph
                    // with the resource diff (which may include outputs copied from the previous
                    // resoure).
                    self.resources
                        .insert(resource_ref, resource_diff.resource.clone());

                    results.noop_count += 1;
                }
                OperationResult::Skipped(reason) => {
                    // A dependency of this resource failed to evaluate. If the resource existed
                    // previously, we will copy the old version into this graph. Otherwise, we will
                    // remove this resource from the graph.
                    if let Some(previous_resource) = resource_diff.previous_resource {
                        self.resources.insert(resource_ref, previous_resource);
                    } else {
                        self.resources.remove(&resource_ref);
                    }

                    results.skipped_count += 1;
                    logger::end_action(format!("Skipped: {}", Paint::yellow(reason)));
                }
                OperationResult::Failed(e) => {
                    // An error occurred while creating or updating the resource. If the
                    // resource existed previously, we will copy the old version into this
                    // graph. Otherwise, we will remove this resource from the graph.
                    if let Some(previous_resource) = resource_diff.previous_resource {
                        self.resources.insert(resource_ref, previous_resource);
                    } else {
                        self.resources.remove(&resource_ref);
                    }

                    failures_count += 1;

                    // TODO: this may need work for formatting
                    logger::end_action(format!("Failed: {}", Paint::red(e)));
                }
            };
        }

        // Iterate over previous resources in reverse order so that leaf resources are removed first
        let mut previous_resource_order = previous_graph.get_topological_order()?;
        previous_resource_order.reverse();

        // TODO: improve error handling for deletes too
        for resource_ref in previous_resource_order.iter() {
            let resource = &previous_graph.get_resource_from_ref(resource_ref).unwrap();

            // If the resource is still in the graph, there is no need to delete the resource
            if self.get_resource_from_ref(resource_ref).is_some() {
                continue;
            }

            let resolved_inputs = previous_graph.resolve_inputs(resource)?.unwrap_or_default();
            let outputs = resource.outputs.clone().unwrap_or_default();

            results.deleted_count += 1;
            logger::start_action(format!(
                "{} Deleting: {} {}",
                Paint::red("-"),
                resource.resource_type,
                resource.id
            ));
            logger::log_changeset(get_changeset(
                &previous_graph.get_inputs_hash(&resolved_inputs)?,
                "",
            ));
            let result = resource_manager.delete(
                &resource.resource_type,
                serde_yaml::to_value(resolved_inputs)
                    .map_err(|e| format!("Failed to serialize inputs: {}", e))?,
                serde_yaml::to_value(outputs)
                    .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
            );

            if let Err(error) = result {
                // An error occurred while deleting the resource. We will copy the old version into
                // the graph.
                // TODO: this may need work for formatting
                logger::end_action(format!("Failed: {}", Paint::red(error)));
                failures_count += 1;
                self.resources
                    .insert(resource_ref.clone(), resource.clone());
            } else {
                logger::end_action("Succeeded");
            }
        }

        if failures_count > 0 {
            Err(format!(
                "Failed {} update(s) while evaluating the resource graph. See above for more details",
                failures_count
            ))
        } else {
            Ok(results)
        }
    }
}
