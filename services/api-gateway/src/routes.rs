use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use user_service::user_client::UserClient;

mod user_service {
    tonic::include_proto!("dubpay.user_service");
}

mod social_service {
    tonic::include_proto!("dubpay.social_service");
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    username: String,
    email: String,
    password: String,
    phone_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    user_id: String,
    token: String,
    message: String,
}

pub async fn register_user(Json(payload): Json<UserRequest>) -> impl IntoResponse {
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

#[derive(Serialize, Deserialize)]
pub struct AddFriendRequest {
    user_id: String,
    target_username: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddFriendResponse {
    success: bool,
    message: String,
}

pub async fn add_friend(Json(payload): Json<AddFriendRequest>) -> impl IntoResponse {}
