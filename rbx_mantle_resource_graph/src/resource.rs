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
