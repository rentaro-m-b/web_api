// このファイルではユースケースメソッドを組み合わせて一つの機能を実装する
// RESTならばリクエスト、レスポンスを管理するのがここになるが、今回はそれはない

use std::sync::Arc;

use async_trait::async_trait;

use crate::usecase::hello::HelloUsecase;

#[async_trait]
pub trait HelloHandler: Send + Sync {
    async fn hello(&self) -> String;
}

pub struct HelloHandlerImpl {
    pub hello_usecase: Arc<dyn HelloUsecase>,
}

#[async_trait]
impl HelloHandler for HelloHandlerImpl {
    async fn hello(&self) -> String {
        self.hello_usecase.hello().await
    }
}
