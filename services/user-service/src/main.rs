use std::env;

use tonic::{transport::Server, Request, Response, Status};

use dubpay::user_server::{User, UserServer};
use dubpay::{
    AuthenticateUserRequest, AuthenticateUserResponse, CheckIfUserExistsRequest,
    CheckIfUserExistsResponse, GetNotificationPreferencesRequest,
    GetNotificationPreferencesResponse, GetPrivacySettingsRequest, GetPrivacySettingsResponse,
    GetUserByIdRequest, GetUserByIdResponse, GetUserByUsernameRequest, GetUserByUsernameResponse,
    GetUserProfileRequest, GetUserProfileResponse, RegisterUserRequest, RegisterUserResponse,
    UpdateUserProfileRequest, UpdateUserProfileResponse,
};

pub mod dubpay {
    tonic::include_proto!("dubpay.user_service");
}

#[derive(Debug, Default)]
pub struct UserService;

#[tonic::async_trait]
impl User for UserService {
    async fn register_user(
        &self,
        _request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn authenticate_user(
        &self,
        _request: Request<AuthenticateUserRequest>,
    ) -> Result<Response<AuthenticateUserResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_user_profile(
        &self,
        _request: Request<GetUserProfileRequest>,
    ) -> Result<Response<GetUserProfileResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn update_user_profile(
        &self,
        _request: Request<UpdateUserProfileRequest>,
    ) -> Result<Response<UpdateUserProfileResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_user_by_id(
        &self,
        _request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserByIdResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_user_by_username(
        &self,
        _request: Request<GetUserByUsernameRequest>,
    ) -> Result<Response<GetUserByUsernameResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn check_if_user_exists(
        &self,
        _request: Request<CheckIfUserExistsRequest>,
    ) -> Result<Response<CheckIfUserExistsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_notification_preferences(
        &self,
        _request: Request<GetNotificationPreferencesRequest>,
    ) -> Result<Response<GetNotificationPreferencesResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_privacy_settings(
        &self,
        _request: Request<GetPrivacySettingsRequest>,
    ) -> Result<Response<GetPrivacySettingsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("SERVICE_PORT").unwrap_or_else(|_| "50053".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let service = UserService;

    println!("User Service listening on {}", addr);

    Server::builder()
        .add_service(UserServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
