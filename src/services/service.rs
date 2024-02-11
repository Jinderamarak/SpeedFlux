use async_trait::async_trait;

#[async_trait]
pub trait Service {
    fn name(&self) -> String;
    async fn execute(&self) -> anyhow::Result<()>;
}
