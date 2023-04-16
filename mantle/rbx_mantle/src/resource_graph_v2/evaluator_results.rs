#[derive(Debug, PartialEq)]
pub enum SkipReason {
    PurchasesNotAllowed,
}

#[derive(Debug, PartialEq)]
pub enum OperationType {
    Create,
    Update,
    Recreate,
    Delete,
    Noop,
    Skip(SkipReason),
}

#[derive(Debug)]
pub enum OperationStatus {
    Success,
    Failure(anyhow::Error),
}
// Ignores error messages
impl PartialEq for OperationStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Success, Self::Success) => true,
            (Self::Failure(_), Self::Failure(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OperationResult {
    pub resource_id: String,
    pub operation_type: OperationType,
    pub status: OperationStatus,
}

#[derive(Default, Debug, PartialEq)]
pub struct EvaluatorResults {
    pub operation_results: Vec<OperationResult>,
}

impl EvaluatorResults {
    pub fn is_empty(&self) -> bool {
        self.operation_results.is_empty()
    }

    pub fn create_succeeded(&mut self, resource_id: &str) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Create,
            status: OperationStatus::Success,
        })
    }
    pub fn create_failed(&mut self, resource_id: &str, error: anyhow::Error) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Create,
            status: OperationStatus::Failure(error),
        })
    }

    pub fn update_succeeded(&mut self, resource_id: &str) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Update,
            status: OperationStatus::Success,
        })
    }
    pub fn update_failed(&mut self, resource_id: &str, error: anyhow::Error) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Update,
            status: OperationStatus::Failure(error),
        })
    }

    pub fn recreate_succeeded(&mut self, resource_id: &str) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Recreate,
            status: OperationStatus::Success,
        })
    }
    pub fn recreate_failed(&mut self, resource_id: &str, error: anyhow::Error) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Recreate,
            status: OperationStatus::Failure(error),
        })
    }

    pub fn delete_succeeded(&mut self, resource_id: &str) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Delete,
            status: OperationStatus::Success,
        })
    }
    pub fn delete_failed(&mut self, resource_id: &str, error: anyhow::Error) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Delete,
            status: OperationStatus::Failure(error),
        })
    }

    pub fn noop(&mut self, resource_id: &str) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Noop,
            status: OperationStatus::Success,
        })
    }

    pub fn skip(&mut self, resource_id: &str, reason: SkipReason) {
        self.operation_results.push(OperationResult {
            resource_id: resource_id.to_owned(),
            operation_type: OperationType::Skip(reason),
            status: OperationStatus::Success,
        })
    }
}
