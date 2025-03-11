use axum::{routing::post, Router};
use std::net::SocketAddr;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/register", post(routes::register_user))
        .route("/add-friend", post(routes::add_friend));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("🚀 API Gateway running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
