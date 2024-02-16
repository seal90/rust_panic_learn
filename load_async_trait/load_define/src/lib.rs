use async_trait::async_trait;

#[async_trait]
pub trait LoadAsyncTrait {
    async fn load_fun(&self)->String;
}