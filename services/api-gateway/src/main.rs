use std::net::SocketAddr;
use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use user_service::user_client::UserClient;

mod user_service {
    tonic::include_proto!("dubpay.user_service");
}

#[derive(Serialize, Deserialize)]
struct UserRequest {
    username: String,
    email: String,
    password: String,
    phone_number: String
}

#[derive(Serialize, Deserialize)]
struct UserResponse {
    user_id: String,
    token: String,
    message: String,
}

async fn register_user(Json(payload): Json<UserRequest>) -> impl IntoResponse{
    // ✅ Connect to user-service inside Docker network
    let mut user_client = UserClient::connect("http://user-service:50051")
        .await
        .unwrap();
    
    println!("Requesting User Service...");

    let response = user_client
        .register_user(tonic::Request::new(user_service::RegisterUserRequest {
            username: payload.username.clone(),
            email: payload.email.clone(),
            password: payload.password.clone(),
            phone_number: payload.phone_number.clone(),
        }))
        .await
        .unwrap();

    // ✅ Return a response to the client
    Json(UserResponse {
        user_id: response.into_inner().user_id,
        token: "fake-jwt-token".to_string(),
        message: "User registered successfully!".to_string(),
    })
}
#[tokio::main]
async fn main() {
    let app= Router::new().route("/register", post(register_user));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("🚀 API Gateway running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}