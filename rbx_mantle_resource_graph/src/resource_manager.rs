use async_trait::async_trait;

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
        price: Option<u32>,
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
        price: Option<u32>,
    ) -> Result<TOutputs, String>;

    async fn delete(
        &self,
        outputs: TOutputs,
        dependency_outputs: Vec<TOutputs>,
    ) -> Result<(), String>;
}
