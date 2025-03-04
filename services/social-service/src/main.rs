use dubpay::social_service_handler_server::{SocialServiceHandler, SocialServiceHandlerServer};
use dubpay::{
    AddFriendRequest, AddFriendResponse, GetActivityFeedRequest, GetActivityFeedResponse,
    CheckFriendConnectionsRequest, CheckFriendConnectionsResponse, AddToActivityFeedRequest,
    AddToActivityFeedResponse, GetFriendRecommendationsRequest, GetFriendRecommendationsResponse,
};
use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod dubpay {
    tonic::include_proto!("dubpay.social_service");
}

#[derive(Debug, Default)]
pub struct SocialService;

#[tonic::async_trait]
impl SocialServiceHandler for SocialService {
    async fn add_friend(
        &self,
        _request: Request<AddFriendRequest>,
    ) -> Result<Response<AddFriendResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_activity_feed(
        &self,
        _request: Request<GetActivityFeedRequest>,
    ) -> Result<Response<GetActivityFeedResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn check_friend_connections(
        &self,
        _request: Request<CheckFriendConnectionsRequest>,
    ) -> Result<Response<CheckFriendConnectionsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn add_to_activity_feed(
        &self,
        _request: Request<AddToActivityFeedRequest>,
    ) -> Result<Response<AddToActivityFeedResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_friend_recommendations(
        &self,
        _request: Request<GetFriendRecommendationsRequest>,
    ) -> Result<Response<GetFriendRecommendationsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("SERVICE_PORT").unwrap_or_else(|_| "50052".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let service = SocialService;

    println!("🚀 SocialService listening on {}", addr);

    Server::builder()
        .add_service(SocialServiceHandlerServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
