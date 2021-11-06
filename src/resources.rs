use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

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

    pub fn add_ref_input(
        &mut self,
        name: &str,
        input_ref_type: &str,
        input_ref_id: &str,
        input_ref_output: &str,
    ) -> &mut Self {
        self.inputs.insert(
            name.to_owned(),
            Input::Ref((
                input_ref_type.to_owned(),
                input_ref_id.to_owned(),
                input_ref_output.to_owned(),
            )),
        );
        self
    }

    pub fn add_ref_input_list(
        &mut self,
        name: &str,
        input_ref_list: &Vec<(&str, &str, &str)>,
    ) -> &mut Self {
        self.inputs.insert(
            name.to_owned(),
            Input::RefList(
                input_ref_list
                    .iter()
                    .map(|(input_ref_type, input_ref_id, input_ref_output)| {
                        (
                            input_ref_type.to_owned().to_owned(),
                            input_ref_id.to_owned().to_owned(),
                            input_ref_output.to_owned().to_owned(),
                        )
                    })
                    .collect(),
            ),
        );
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

    fn get_output_from_input_ref(&self, input_ref: &InputRef) -> Result<OutputValue, String> {
        if let Some(outputs) = &self.outputs {
            let value = outputs
                .get(&output_name_from_input_ref(input_ref))
                .ok_or(format!("No output with ref: {:?}", input_ref))?;
            Ok(value.clone())
        } else {
            return Err(format!(
                "Resource {}.{} has no outputs",
                self.resource_type, self.id
            ));
        }
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
    match input_ref {
        (input_ref_type, input_ref_id, _) => (input_ref_type.clone(), input_ref_id.clone()),
    }
}

pub fn output_name_from_input_ref(input_ref: &InputRef) -> String {
    match input_ref {
        (_, _, input_ref_output) => input_ref_output.clone(),
    }
}
pub trait ResourceManagerBackend {
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

pub struct ResourceManager {
    implementation: Box<dyn ResourceManagerBackend>,
}

impl ResourceManager {
    pub fn new(implementation: Box<dyn ResourceManagerBackend>) -> Self {
        ResourceManager { implementation }
    }

    fn create(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
    ) -> Result<Option<serde_yaml::Value>, String> {
        self.implementation.create(resource_type, resource_inputs)
    }

    fn update(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
        resource_outputs: serde_yaml::Value,
    ) -> Result<Option<serde_yaml::Value>, String> {
        self.implementation
            .update(resource_type, resource_inputs, resource_outputs)
    }

    fn delete(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
        resource_outputs: serde_yaml::Value,
    ) -> Result<(), String> {
        self.implementation
            .delete(resource_type, resource_inputs, resource_outputs)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceGraph {
    pub resources: HashMap<ResourceRef, Resource>,
}

impl ResourceGraph {
    pub fn new(resources: &Vec<Resource>) -> Self {
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
        resources.sort_by(|a, b| a.get_ref().cmp(&b.get_ref()));
        resources
    }

    pub fn get_resource_from_ref(&self, resource_ref: &ResourceRef) -> Option<Resource> {
        self.resources
            .get(resource_ref)
            .map(|resource| resource.clone())
    }

    fn get_resource_from_input_ref(&self, input_ref: &InputRef) -> Option<Resource> {
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
                    let referenced_resource = self
                        .get_resource_from_input_ref(value_ref)
                        .ok_or(format!("Reference not found: {:?}", value_ref))?;
                    if referenced_resource.outputs.is_none() {
                        return Ok(None);
                    }
                    resolved_inputs.insert(
                        name.clone(),
                        referenced_resource.get_output_from_input_ref(value_ref)?,
                    );
                }
                Input::RefList(ref_list) => {
                    let mut resolved_values = Vec::new();
                    for value_ref in ref_list {
                        let referenced_resource = self
                            .get_resource_from_input_ref(value_ref)
                            .ok_or(format!("Reference not found: {:?}", value_ref))?;
                        if referenced_resource.outputs.is_none() {
                            return Ok(None);
                        }
                        resolved_values
                            .push(referenced_resource.get_output_from_input_ref(value_ref)?);
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

    fn get_resource_diffs(
        &self,
        previous_graph: &ResourceGraph,
    ) -> Result<Vec<ResourceDiff>, String> {
        let mut resource_diffs: Vec<ResourceDiff> = Vec::new();
        for (resource_ref, resource) in &self.resources {
            match previous_graph.get_resource_from_ref(resource_ref) {
                Some(previous_resource) => {
                    let previous_inputs = previous_graph
                        .resolve_inputs(&previous_resource)?
                        .ok_or("Previous graph should be complete.")?;
                    resource_diffs.push(ResourceDiff {
                        previous_hash: Some(previous_graph.get_inputs_hash(&previous_inputs)?),
                        resource: Resource {
                            id: resource.id.clone(),
                            inputs: resource.inputs.clone(),
                            outputs: previous_resource.outputs.clone(),
                            resource_type: resource.resource_type.clone(),
                        },
                    });
                }
                None => {
                    resource_diffs.push(ResourceDiff {
                        previous_hash: None,
                        resource: resource.clone(),
                    });
                }
            };
        }
        Ok(resource_diffs)
    }

    pub fn resolve(
        &mut self,
        resource_manager: &mut ResourceManager,
        previous_graph: &ResourceGraph,
    ) -> Result<(), String> {
        // TODO: Something more elegant than this loop (i.e. build dependency graph)
        // TODO: Catch edge circular dependencies (currently inifinte loops)
        let mut resource_diffs = self.get_resource_diffs(previous_graph)?;
        while !resource_diffs.is_empty() {
            let mut next_resource_diffs: Vec<ResourceDiff> = Vec::new();

            for ResourceDiff {
                resource,
                previous_hash,
            } in resource_diffs.iter()
            {
                // println!("Resolving resource {:?}", resource.get_ref());
                let resolved_inputs = self.resolve_inputs(&resource)?;
                match resolved_inputs {
                    None => {
                        next_resource_diffs.push(ResourceDiff {
                            resource: resource.clone(),
                            previous_hash: previous_hash.clone(),
                        });
                    }
                    Some(inputs) => {
                        let outputs = match previous_hash {
                            None => {
                                Some(resource_manager.create(
                                    &resource.resource_type,
                                    serde_yaml::to_value(&inputs).map_err(|e| {
                                        format!("Failed to serialize inputs: {}", e)
                                    })?,
                                )?)
                            }
                            Some(previous_hash)
                                if previous_hash.to_owned() != self.get_inputs_hash(&inputs)? =>
                            {
                                let outputs = resource.outputs.clone().unwrap_or_default();
                                Some(resource_manager.update(
                                    &resource.resource_type,
                                    serde_yaml::to_value(inputs).map_err(|e| {
                                        format!("Failed to serialize inputs: {}", e)
                                    })?,
                                    serde_yaml::to_value(outputs).map_err(|e| {
                                        format!("Failed to serialize inputs: {}", e)
                                    })?,
                                )?)
                            }
                            _ => None,
                        };
                        let mut new_resource = resource.clone();
                        if let Some(Some(outputs)) = outputs {
                            let outputs =
                                serde_yaml::from_value::<BTreeMap<String, OutputValue>>(outputs)
                                    .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;
                            for (key, value) in outputs {
                                new_resource.add_output(&key, &value)?;
                            }
                        }
                        self.resources.insert(new_resource.get_ref(), new_resource);
                    }
                };
            }

            resource_diffs.clear();
            resource_diffs.extend(next_resource_diffs);
        }

        for (resource_ref, resource) in previous_graph.resources.iter() {
            let resolved_inputs = previous_graph.resolve_inputs(&resource)?;
            match self.get_resource_from_ref(resource_ref) {
                None => {
                    let outputs = resource.outputs.clone().unwrap_or_default();
                    resource_manager.delete(
                        &resource.resource_type,
                        serde_yaml::to_value(resolved_inputs)
                            .map_err(|e| format!("Failed to serialize inputs: {}", e))?,
                        serde_yaml::to_value(outputs)
                            .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
