use dubpay::transaction_handler_server::{TransactionHandler, TransactionHandlerServer};
use dubpay::{
    notification_handler_client, ConfirmRequestRequest, ConfirmRequestResponse,
    RejectRequestRequest, RejectRequestResponse, RequestMoneyRequest, RequestMoneyResponse,
    SendPaymentRequest, SendPaymentResponse,
};
use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod dubpay {
    tonic::include_proto!("dubpay.transaction");
    tonic::include_proto!("dubpay.notification");
}

#[derive(Debug, Default)]
pub struct TransactionService;

#[tonic::async_trait]
impl TransactionHandler for TransactionService {
    async fn request_money(
        &self,
        _request: Request<RequestMoneyRequest>,
    ) -> Result<Response<RequestMoneyResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn confirm_request(
        &self,
        _request: Request<ConfirmRequestRequest>,
    ) -> Result<Response<ConfirmRequestResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn reject_request(
        &self,
        _request: Request<RejectRequestRequest>,
    ) -> Result<Response<RejectRequestResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn send_payment(
        &self,
        request: Request<SendPaymentRequest>,
    ) -> Result<Response<SendPaymentResponse>, Status> {
        let request_content = request.into_inner();
        let from = request_content.from_user_id;
        let to = request_content.to_user_id;
        let amount = request_content.amount;
        let public = request_content.public;
        let description = request_content.description;

        println!(
            "Received payment request from {} to {} for {}, public: {}, description: {}",
            from, to, amount, public, description
        );

        let mut notification_handler_client =
            notification_handler_client::NotificationHandlerClient::connect(
                "http://notification-service:50051",
            )
            .await
            .unwrap();

        let notification_request = tonic::Request::new(dubpay::SendNotificationRequest {
            user_id: to,
            message: format!("You received a payment of {} from {}", amount, from),
            timestamp: Some(prost_types::Timestamp {
                seconds: 0,
                nanos: 0,
            }),
        });

        let response = notification_handler_client
            .send_notification(notification_request)
            .await
            .unwrap();

        println!("Notification response: {:?}", response.into_inner().message);

        return Ok(Response::new(SendPaymentResponse {
            id: "1".to_string(),
            message: "success".to_string(),
        }));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("SERVICE_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let service = TransactionService;

    println!("🚀 TransactionService listening on {}", addr);

    Server::builder()
        .add_service(TransactionHandlerServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
