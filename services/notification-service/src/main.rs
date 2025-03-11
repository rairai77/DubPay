use dubpay::notification_handler_server::{NotificationHandler, NotificationHandlerServer};
use dubpay::{SendNotificationRequest, SendNotificationResponse};
use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod dubpay {
    tonic::include_proto!("dubpay.notification");
}

#[derive(Debug, Default)]
pub struct NotificationService;

#[tonic::async_trait]
impl NotificationHandler for NotificationService {
    async fn send_notification(
        &self,
        request: Request<SendNotificationRequest>,
    ) -> Result<Response<SendNotificationResponse>, Status> {
        let request_content = request.into_inner();
        let user = request_content.user_id;
        let message = request_content.message;
        let timestamp = request_content.timestamp.unwrap();

        println!(
            "Sending notification to user {} at {} with message: {}",
            user, timestamp, message
        );

        return Ok(Response::new(SendNotificationResponse {
            id: "1".to_string(),
            message: "success".to_string(),
        }));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("SERVICE_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let service = NotificationService;

    println!("🚀 NotificationService listening on {}", addr);

    Server::builder()
        .add_service(NotificationHandlerServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
