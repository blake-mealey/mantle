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

    pub fn add_value_stub_input(&mut self, name: &str) -> &mut Self {
        self.inputs
            .insert(name.to_owned(), Input::Value(serde_yaml::Value::Null));
        self
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

    pub fn add_output<T>(&mut self, name: &str, output_value: &T) -> Result<&mut Self, String>
    where
        T: serde::Serialize,
    {
        if self.outputs.is_none() {
            self.outputs = Some(BTreeMap::new());
        }
        let serialized_value = serde_yaml::to_value(output_value)
            .map_err(|e| format!("Failed to serialize output value:\n\t{}", e))?;
        self.outputs
            .as_mut()
            .unwrap()
            .insert(name.to_owned(), serialized_value);
        Ok(self)
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

    pub fn evaluate<TManager>(
        &mut self,
        previous_graph: &ResourceGraph,
        resource_manager: &mut TManager,
    ) -> Result<EvaluateResults, String>
    where
        TManager: ResourceManager,
    {
        let resource_order = self.get_topological_order()?;

        let mut results: EvaluateResults = Default::default();
        let mut failures_count = 0;

        for resource_ref in resource_order {
            let ResourceDiff {
                resource,
                previous_resource,
                previous_hash,
            } = self.get_resource_diff(previous_graph, &resource_ref)?;

            let resolved_inputs = self.resolve_inputs(&resource)?;
            if let Some(inputs) = resolved_inputs {
                // We can proceed to evaluate this resource
                let inputs_hash = self.get_inputs_hash(&inputs)?;
                let outputs = match previous_hash {
                    None => {
                        // This resource is new
                        results.created_count += 1;
                        logger::start_action(format!(
                            "{} Creating: {} {}",
                            Paint::green("+"),
                            resource.resource_type,
                            resource.id
                        ));
                        logger::log_changeset(get_changeset("", &inputs_hash));
                        Some(
                            resource_manager.create(
                                &resource.resource_type,
                                serde_yaml::to_value(&inputs)
                                    .map_err(|e| format!("Failed to serialize inputs: {}", e))?,
                            ),
                        )
                    }
                    Some(previous_hash) if previous_hash != inputs_hash => {
                        // This resource has changed
                        results.updated_count += 1;
                        logger::start_action(format!(
                            "{} Updating: {} {}",
                            Paint::yellow("~"),
                            resource.resource_type,
                            resource.id
                        ));
                        logger::log_changeset(get_changeset(&previous_hash, &inputs_hash));
                        let outputs = resource.outputs.clone().unwrap_or_default();
                        Some(
                            resource_manager.update(
                                &resource.resource_type,
                                serde_yaml::to_value(inputs)
                                    .map_err(|e| format!("Failed to serialize inputs: {}", e))?,
                                serde_yaml::to_value(outputs)
                                    .map_err(|e| format!("Failed to serialize inputs: {}", e))?,
                            ),
                        )
                    }
                    _ => {
                        results.noop_count += 1;
                        None
                    }
                };

                if let Some(outputs) = outputs {
                    // We attempted to create or update the resource
                    if let Ok(outputs) = outputs {
                        // We successfully created or updated the resource

                        if let Some(outputs) = outputs {
                            // Apply the outputs to the resource
                            logger::end_action_with_results(
                                "Succeeded with outputs:",
                                format_inputs_hash(
                                    &serde_yaml::to_string(&outputs).map_err(|e| {
                                        format!("Failed to serialize outputs: {}", e)
                                    })?,
                                ),
                            );
                            let mut resource_with_outputs = resource.clone();
                            let outputs =
                                serde_yaml::from_value::<BTreeMap<String, OutputValue>>(outputs)
                                    .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;
                            // TODO: how to handle deserialization errors?
                            for (key, value) in outputs {
                                resource_with_outputs.add_output(&key, &value)?;
                            }
                            self.resources.insert(resource_ref, resource_with_outputs);
                        } else {
                            logger::end_action("Succeeded");
                        }
                    } else {
                        // An error occurred while creating or updating the resource. If the
                        // resource existed previously, we will copy the old version into this
                        // graph. Otherwise, we will remove this resource from the graph.
                        // TODO: this may need work for formatting
                        logger::end_action(format!("Failed: {}", Paint::red(outputs.unwrap_err())));
                        failures_count += 1;
                        if let Some(previous_resource) = previous_resource {
                            self.resources.insert(resource_ref, previous_resource);
                        } else {
                            self.resources.remove(&resource_ref);
                        }
                    }
                } else {
                    // There was no need to create or update the resource. We will update the graph
                    // with the resource diff (which may include outputs copied from the previous
                    // resoure).
                    self.resources.insert(resource_ref, resource.clone());
                }
            } else {
                // A dependency of this resource failed to evaluate. If the resource existed
                // previously, we will copy the old version into this graph. Otherwise, we will
                // remove this resource from the graph.
                if let Some(previous_resource) = previous_resource {
                    self.resources.insert(resource_ref, previous_resource);
                } else {
                    self.resources.remove(&resource_ref);
                }
            }
        }

        for (resource_ref, resource) in previous_graph.resources.iter() {
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
                "Failed {} updates while evaluating the resource graph. See above for more details",
                failures_count
            ))
        } else {
            Ok(results)
        }
    }
}
