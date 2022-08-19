use rbx_mantle_resource_graph::resource::{Resource, ResourceId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RobloxResource<TInputs, TOutputs> {
    id: ResourceId,
    inputs: TInputs,
    outputs: Option<TOutputs>,
    dependencies: Vec<ResourceId>,
}

impl<TInputs: Clone + Serialize, TOutputs: Clone + Serialize> RobloxResource<TInputs, TOutputs> {
    pub fn new(
        id: &str,
        inputs: TInputs,
        dependencies: &[&RobloxResource<TInputs, TOutputs>],
    ) -> Self {
        Self {
            id: id.to_owned(),
            inputs,
            outputs: None,
            dependencies: dependencies.iter().map(|d| d.get_id()).collect(),
        }
    }

    pub fn existing(
        id: &str,
        inputs: TInputs,
        outputs: TOutputs,
        dependencies: &[&RobloxResource<TInputs, TOutputs>],
    ) -> Self {
        Self {
            id: id.to_owned(),
            inputs,
            outputs: Some(outputs),
            dependencies: dependencies.iter().map(|d| d.get_id()).collect(),
        }
    }

    pub fn add_dependency(&mut self, dependency: &RobloxResource<TInputs, TOutputs>) -> &mut Self {
        self.dependencies.push(dependency.get_id());
        self
    }
}

impl<TInputs: Clone + Serialize, TOutputs: Clone + Serialize> Resource<TInputs, TOutputs>
    for RobloxResource<TInputs, TOutputs>
{
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_inputs_hash(&self) -> String {
        // TODO: Should we separate hashes from displays?
        let hash = serde_yaml::to_string(&self.inputs)
            .map_err(|e| format!("Failed to compute inputs hash\n\t{}", e))
            .unwrap();
        if hash.is_empty() {
            ""
        } else {
            // We remove first 4 characters to remove "---\n", and we trim the end to remove "\n"
            hash[4..].trim_end()
        }
        .to_owned()
    }

    fn get_outputs_hash(&self) -> String {
        // TODO: Should we separate hashes from displays?
        let hash = serde_yaml::to_string(&self.outputs)
            .map_err(|e| format!("Failed to compute outputs hash\n\t{}", e))
            .unwrap();
        if hash.is_empty() {
            ""
        } else {
            // We remove first 4 characters to remove "---\n", and we trim the end to remove "\n"
            hash[4..].trim_end()
        }
        .to_owned()
    }

    fn get_inputs(&self) -> TInputs {
        self.inputs.clone()
    }

    fn get_outputs(&self) -> Option<TOutputs> {
        self.outputs.clone()
    }

    fn get_dependencies(&self) -> Vec<ResourceId> {
        self.dependencies.clone()
    }

    fn set_outputs(&mut self, outputs: TOutputs) {
        self.outputs = Some(outputs);
    }
}
