use tonic::{transport::Server, Request, Response, Status};

use dubpay::user_server::{User, UserServer};
use dubpay::{
    RegisterUserRequest, RegisterUserResponse,
    AuthenticateUserRequest, AuthenticateUserResponse,
    GetUserProfileRequest, GetUserProfileResponse,
    UpdateUserProfileRequest, UpdateUserProfileResponse,
    GetUserByIDRequest, GetUserByIDResponse,
    GetUserByUsernameRequest, GetUserByUsernameResponse,
    CheckIfUserExistsRequest, CheckIfUserExistsResponse,
    GetNotificationPreferencesRequest, GetNotificationPreferencesResponse,
    GetPrivacySettingsRequest, GetPrivacySettingsResponse
};

pub mod user_server {
    tonic::include_proto!("dubpay.user-service");
}

#[derive(Debug, Default)]
pub struct UserService;

#[tonic::async_trait]
impl Wallet for UserService {
    async fn register_user(
        &self,
        _request: Request<RegisterUserRequest>, 
    ) -> Result<Response<RegisterUserResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn auth_user(
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
        _request: Request<GetUserByIDRequest>, 
    ) -> Result<Response<GetUserByIDResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_user_by_username(
        &self,
        _request: Request<GetUserByUsernameRequest>, 
    ) -> Result<Response<GetUserByUsernameResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn user_exists(
        &self,
        _request: Request<CheckIfUserExistsRequest>, 
    ) -> Result<Response<CheckIfUserExistsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_notifications_preferences(
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
    let addr = "[::1]:50053".parse()?;
    let service = UserService;

    println!("User Service listening on {}", addr);

    Server::builder()
        .add_service(UserServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}


