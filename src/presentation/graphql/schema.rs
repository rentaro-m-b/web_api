use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::GraphQLRequest;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use axum_macros::debug_handler;

use crate::presentation::handler::hello::HelloHandler;

// 1. 造体Queryにメソッドを実装
// 2. スキーマを生成
// 3. スキーマを実行

// 1. レポジトリ構造体にメソッドを実装
// 2. ユースケース構造体にメソッドを実装
// 3. ハンドラー構造体にメソッドを実装

pub struct Query {
    pub hello_handler: Arc<dyn HelloHandler>,
}

#[Object]
impl Query {
    async fn hello(&self) -> String {
        self.hello_handler.hello().await
    }
}

#[debug_handler]
pub async fn graphql_execute(
    Extension(schema): Extension<Arc<Schema<Query, EmptyMutation, EmptySubscription>>>,
    req: GraphQLRequest
) -> impl IntoResponse
{
    let res = schema.execute(req.into_inner()).await;

    (StatusCode::OK, Json(res))
}

pub async fn new_schema(query: Query) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::new(query, EmptyMutation, EmptySubscription)
}
