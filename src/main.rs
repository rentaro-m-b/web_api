use tokio;
use web_api::routes::new_routers;

#[tokio::main]
async fn main() {
    let app = new_routers().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
