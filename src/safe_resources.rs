use std::{collections::HashMap, fmt::Display, hash::Hash};

// defined externally

#[derive(Clone)]
enum ImplInputs {
    Experience { name: String },
    Asset { name: String },
}

#[derive(Clone)]
enum ImplOutputs {
    Experience { asset_id: u64 },
    Asset { asset_id: u64 },
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum ImplType {
    Experience,
    Asset,
}
impl Display for ImplType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ImplType::Experience => "experience",
            ImplType::Asset => "asset",
        })
    }
}

#[derive(Clone)]
struct ImplResource {
    resource_type: ImplType,
    id: String,
    inputs: ImplInputs,
    outputs: ImplOutputs,
    dependencies: Vec<ResourceRef<ImplType>>,
}

impl Resource<ImplType, ImplInputs, ImplOutputs> for ImplResource {
    fn get_type(&self) -> ImplType {
        self.resource_type.clone()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_inputs_hash(&self) -> String {
        todo!("hash the inputs")
    }

    fn get_inputs(&self) -> ImplInputs {
        self.inputs.clone()
    }

    fn get_outputs(&self) -> ImplOutputs {
        self.outputs.clone()
    }

    fn get_dependencies(&self) -> Vec<ResourceRef<ImplType>> {
        self.dependencies.clone()
    }
}

struct ImplResourceManager {}

impl ResourceManager<ImplType, ImplInputs, ImplOutputs> for ImplResourceManager {
    fn create(
        &self,
        resource_type: ImplType,
        inputs: ImplInputs,
        dependency_outputs: HashMap<ResourceRef<ImplType>, ImplOutputs>,
    ) -> ImplOutputs {
        match (resource_type, inputs) {
            (ImplType::Experience, ImplInputs::Experience { name }) => {
                if let ImplOutputs::Experience {
                    asset_id: experience_id,
                } = get_first_output(&dependency_outputs, ImplType::Experience)
                {
                    let thumbnail_ids = get_outputs(&dependency_outputs, ImplType::Asset)
                        .iter()
                        .filter_map(|d| match d {
                            ImplOutputs::Asset { asset_id } => Some(asset_id),
                            _ => None,
                        })
                        .collect::<Vec<&u64>>();

                    // TODO: make requests and return outputs
                }
                panic!("missing expected dependency outputs");
            }
            _ => {}
        }
        unimplemented!()
    }
    fn update(
        &self,
        resource_type: ImplType,
        inputs: ImplInputs,
        outputs: ImplOutputs,
        dependency_outputs: HashMap<ResourceRef<ImplType>, ImplOutputs>,
    ) -> ImplOutputs {
        match (resource_type, inputs, outputs) {
            (
                ImplType::Experience,
                ImplInputs::Experience { name },
                ImplOutputs::Experience { asset_id },
            ) => {}
            _ => {}
        }
        unimplemented!()
    }
}

// internals
fn get_first_output<TType, TOutputs>(
    dependency_outputs: &HashMap<ResourceRef<TType>, TOutputs>,
    resource_type: TType,
) -> &TOutputs
where
    TType: Clone,
    TType: Hash,
    TType: Eq,
    TType: Display,
{
    dependency_outputs
        .iter()
        .find_map(|(key, value)| {
            if key.0 == resource_type {
                Some(value)
            } else {
                None
            }
        })
        .expect(&format!(
            "Missing required dependency output of type {}",
            resource_type
        ))
}

fn get_outputs<TType, TOutputs>(
    dependency_outputs: &HashMap<ResourceRef<TType>, TOutputs>,
    resource_type: TType,
) -> Vec<&TOutputs>
where
    TType: Clone,
    TType: Hash,
    TType: Eq,
{
    dependency_outputs
        .iter()
        .filter_map(|(key, value)| {
            if key.0 == resource_type {
                Some(value)
            } else {
                None
            }
        })
        .collect()
}

type ResourceRef<TType> = (TType, String);

trait Resource<TType, TInputs, TOutputs> {
    fn get_type(&self) -> TType;
    fn get_id(&self) -> String;
    fn get_inputs_hash(&self) -> String;
    fn get_inputs(&self) -> TInputs;
    fn get_outputs(&self) -> TOutputs;
    fn get_dependencies(&self) -> Vec<ResourceRef<TType>>;
}

trait ResourceManager<TType, TInputs, TOutputs>
where
    TType: Hash,
    TType: Eq,
    TType: Clone,
{
    fn create(
        &self,
        resource_type: TType,
        inputs: TInputs,
        dependency_outputs: HashMap<ResourceRef<TType>, TOutputs>,
    ) -> TOutputs;
    fn update(
        &self,
        resource_type: TType,
        inputs: TInputs,
        outputs: TOutputs,
        dependency_outputs: HashMap<ResourceRef<TType>, TOutputs>,
    ) -> TOutputs;
}

fn get_dependency_outputs<TResource, TType, TInputs, TOutputs>(
    resources: &HashMap<ResourceRef<TType>, TResource>,
    dependencies: Vec<ResourceRef<TType>>,
) -> Option<HashMap<ResourceRef<TType>, TOutputs>>
where
    TType: Hash,
    TType: Eq,
    TResource: Resource<TType, TInputs, TOutputs>,
{
    let mut dependency_outputs: HashMap<ResourceRef<TType>, TOutputs> = HashMap::new();
    for dependency in dependencies {
        let resource = resources.get(&dependency);
        if let Some(resource) = resource {
            dependency_outputs.insert(dependency, resource.get_outputs());
        } else {
            return None;
        }
    }
    Some(dependency_outputs)
}

fn evaluate<TManager, TResource, TType, TInputs, TOutputs>(
    manager: &TManager,
    resources: &HashMap<ResourceRef<TType>, TResource>,
) where
    TManager: ResourceManager<TType, TInputs, TOutputs>,
    TResource: Resource<TType, TInputs, TOutputs>,
    TType: Clone,
    TType: Hash,
    TType: Eq,
{
    for resource in resources.values() {
        if let Some(dependency_outputs) =
            get_dependency_outputs(resources, resource.get_dependencies())
        {
            manager.create(
                resource.get_type(),
                resource.get_inputs(),
                dependency_outputs,
            );
        }
    }
}
