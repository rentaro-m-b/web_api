use async_trait::async_trait;

#[async_trait]
pub trait HelloUsecase: Send + Sync {
    async fn hello(&self) -> String;
}

pub struct HelloUsecaseImpl {}

#[async_trait]
impl HelloUsecase for HelloUsecaseImpl {
    async fn hello(&self) -> String {
        "hello, Rust".to_string()
    }
}
