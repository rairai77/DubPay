use dubpay::notification_handler_server::{NotificationHandler, NotificationHandlerServer};
use dubpay::{SendNotificationRequest, SendNotificationResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod dubpay{
    tonic::include_proto!("dubpay.notification");
}

#[derive(Debug, Default)]
pub struct NotificationService;

#[tonic::async_trait]
impl NotificationHandler for NotificationService {
    async fn send_notification(
        &self,
        _request: Request<SendNotificationRequest>,
    ) -> Result<Response<SendNotificationResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let service = NotificationService;

    println!("🚀 NotificationService listening on {}", addr);

    Server::builder()
        .add_service(NotificationHandlerServer::new(service))
        .serve(addr)
        .await?;
    
    Ok(())
}