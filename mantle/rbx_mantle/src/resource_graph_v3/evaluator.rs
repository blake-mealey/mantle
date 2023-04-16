use crate::resources_v2::ResourceGroup;

use super::{evaluator_results::EvaluatorResults, ResourceGraph};

pub struct Evaluator<'a> {
    previous_graph: &'a ResourceGraph,
    desired_graph: &'a ResourceGraph,
    next_graph: ResourceGraph,

    results: EvaluatorResults,
}

impl<'a> Evaluator<'a> {
    pub fn new(previous_graph: &'a ResourceGraph, desired_graph: &'a ResourceGraph) -> Self {
        Self {
            previous_graph,
            desired_graph,
            next_graph: ResourceGraph::default(),
            results: EvaluatorResults::default(),
        }
    }

    pub async fn evaluate(
        &'a mut self,
    ) -> anyhow::Result<(&'a EvaluatorResults, &'a ResourceGraph)> {
        if !self.results.is_empty() {
            return anyhow::Result::Err(anyhow::Error::msg(
                "A graph evaluator can only be used once.",
            ));
        }

        self.delete_removed_resources().await?;
        self.create_or_update_added_or_changed_resources().await?;

        Ok((&self.results, &self.next_graph))
    }

    async fn delete_removed_resources(&mut self) -> anyhow::Result<()> {
        let mut previous_resources = self.previous_graph.topological_order()?;
        previous_resources.reverse();

        for resource in previous_resources.into_iter() {
            if self.desired_graph.contains(resource.id()) {
                continue;
            }

            println!("Deleting: {}", resource.id());

            let mut next_resource = resource.clone();

            match next_resource.delete().await {
                Ok(()) => self.results.delete_succeeded(resource.id()),
                Err(error) => {
                    self.results.delete_failed(resource.id(), error);
                    self.next_graph.insert(next_resource);
                }
            }
        }

        Ok(())
    }

    async fn create_or_update_added_or_changed_resources(&mut self) -> anyhow::Result<()> {
        let desired_resources = self.desired_graph.topological_order()?;

        for desired_resource in desired_resources.into_iter() {
            if let Some(previous_resource) = self.previous_graph.get(desired_resource.id()) {
                match desired_resource.next(&self.previous_graph, &self.next_graph) {
                    Ok(mut next_resource) => {
                        if *previous_resource == next_resource {
                            self.results.noop(next_resource.id());
                            self.next_graph.insert(next_resource);
                        } else {
                            match next_resource.update().await {
                                Ok(()) => {
                                    self.results.update_succeeded(next_resource.id());
                                    self.next_graph.insert(next_resource);
                                }
                                Err(error) => {
                                    self.results.update_failed(next_resource.id(), error);
                                    self.next_graph.insert(previous_resource.clone());
                                }
                            }
                        }
                    }
                    Err(error) => self.results.update_failed(desired_resource.id(), error),
                }
            } else {
                match desired_resource.next(&self.previous_graph, &self.next_graph) {
                    Ok(mut next_resource) => match next_resource.create().await {
                        Ok(()) => {
                            self.results.create_succeeded(next_resource.id());
                            self.next_graph.insert(next_resource);
                        }
                        Err(error) => {
                            self.results.create_failed(next_resource.id(), error);
                        }
                    },
                    Err(error) => self.results.create_failed(desired_resource.id(), error),
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        resource_graph_v3::{
            evaluator::Evaluator,
            evaluator_results::{
                EvaluatorResults, OperationResult, OperationStatus, OperationType,
            },
            ResourceGraph,
        },
        resources_v2::{
            experience::{Experience, ExperienceInputs, ExperienceOutputs},
            place::{Place, PlaceInputs, PlaceOutputs},
            RbxResource,
        },
    };
    use pretty_assertions::assert_eq;

    #[tokio::test]
    pub async fn create_resources() {
        let mut desired_graph = ResourceGraph::default();
        let desired_experience = Experience {
            id: "experience_singleton".to_owned(),
            inputs: ExperienceInputs { group_id: None },
            outputs: None,
        };
        let desired_start_place = Place {
            id: "place_start".to_owned(),
            inputs: PlaceInputs { is_start: true },
            outputs: None,
            experience: desired_experience.clone(),
        };
        desired_graph.insert(RbxResource::Place(desired_start_place));
        let desired_other_place = Place {
            id: "place_other".to_owned(),
            inputs: PlaceInputs { is_start: false },
            outputs: None,
            experience: desired_experience.clone(),
        };
        desired_graph.insert(RbxResource::Place(desired_other_place));
        desired_graph.insert(RbxResource::Experience(desired_experience));

        let previous_graph = ResourceGraph::default();

        let mut evaluator = Evaluator::new(&previous_graph, &desired_graph);
        let (results, _next_graph) = evaluator.evaluate().await.unwrap();

        assert_eq!(
            *results,
            EvaluatorResults {
                operation_results: vec![
                    OperationResult {
                        resource_id: "experience_singleton".to_owned(),
                        operation_type: OperationType::Create,
                        status: OperationStatus::Success
                    },
                    OperationResult {
                        resource_id: "place_start".to_owned(),
                        operation_type: OperationType::Create,
                        status: OperationStatus::Success
                    },
                    OperationResult {
                        resource_id: "place_other".to_owned(),
                        operation_type: OperationType::Create,
                        status: OperationStatus::Success
                    }
                ]
            }
        );
    }

    #[tokio::test]
    pub async fn update_resources_noop() {
        let mut previous_graph = ResourceGraph::default();
        let previous_experience = Experience {
            id: "experience_singleton".to_owned(),
            inputs: ExperienceInputs { group_id: None },
            outputs: Some(ExperienceOutputs {
                asset_id: 1,
                start_place_id: 2,
            }),
        };
        let previous_start_place = Place {
            id: "place_start".to_owned(),
            inputs: PlaceInputs { is_start: true },
            outputs: Some(PlaceOutputs { asset_id: 2 }),
            experience: previous_experience.clone(),
        };
        previous_graph.insert(RbxResource::Place(previous_start_place));
        previous_graph.insert(RbxResource::Experience(previous_experience));

        let mut desired_graph = ResourceGraph::default();
        let desired_experience = Experience {
            id: "experience_singleton".to_owned(),
            inputs: ExperienceInputs { group_id: None },
            outputs: None,
        };
        let desired_start_place = Place {
            id: "place_start".to_owned(),
            inputs: PlaceInputs { is_start: true },
            outputs: None,
            experience: desired_experience.clone(),
        };
        desired_graph.insert(RbxResource::Place(desired_start_place));
        desired_graph.insert(RbxResource::Experience(desired_experience));

        let mut evaluator = Evaluator::new(&previous_graph, &desired_graph);
        let (results, _next_graph) = evaluator.evaluate().await.unwrap();

        assert_eq!(
            *results,
            EvaluatorResults {
                operation_results: vec![
                    OperationResult {
                        resource_id: "experience_singleton".to_owned(),
                        operation_type: OperationType::Noop,
                        status: OperationStatus::Success
                    },
                    OperationResult {
                        resource_id: "place_start".to_owned(),
                        operation_type: OperationType::Noop,
                        status: OperationStatus::Success
                    }
                ]
            }
        );
    }

    #[tokio::test]
    pub async fn update_resources_changes() {
        let mut previous_graph = ResourceGraph::default();
        let previous_experience = Experience {
            id: "experience_singleton".to_owned(),
            inputs: ExperienceInputs { group_id: None },
            outputs: None,
        };
        let previous_start_place = Place {
            id: "place_start".to_owned(),
            inputs: PlaceInputs { is_start: true },
            outputs: None,
            experience: previous_experience.clone(),
        };
        previous_graph.insert(RbxResource::Place(previous_start_place));
        previous_graph.insert(RbxResource::Experience(previous_experience));

        let mut desired_graph = ResourceGraph::default();
        let desired_experience = Experience {
            id: "experience_singleton".to_owned(),
            inputs: ExperienceInputs {
                group_id: Some(123),
            },
            outputs: None,
        };
        let desired_start_place = Place {
            id: "place_start".to_owned(),
            inputs: PlaceInputs { is_start: true },
            outputs: None,
            experience: desired_experience.clone(),
        };
        desired_graph.insert(RbxResource::Place(desired_start_place));
        desired_graph.insert(RbxResource::Experience(desired_experience));

        let mut evaluator = Evaluator::new(&previous_graph, &desired_graph);
        let (results, _next_graph) = evaluator.evaluate().await.unwrap();

        assert_eq!(
            *results,
            EvaluatorResults {
                operation_results: vec![
                    OperationResult {
                        resource_id: "experience_singleton".to_owned(),
                        operation_type: OperationType::Update,
                        status: OperationStatus::Success
                    },
                    OperationResult {
                        resource_id: "place_start".to_owned(),
                        operation_type: OperationType::Update,
                        status: OperationStatus::Success
                    }
                ]
            }
        );
    }
}
