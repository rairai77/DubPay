use dubpay::transaction_service_server::TransactionService;
use dubpay::transaction_service_server::TransactionServiceServer;
use dubpay::{
    ConfirmRequestRequest, ConfirmRequestResponse, ErrorMessage, RequestMoneyRequest,
    RequestMoneyResponse, SendPaymentRequest, SendPaymentResponse, SuccessMessage,
};
use prost_types::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{transport::Server, Request, Response, Status};

pub mod dubpay {
    tonic::include_proto!("dubpay");
}

#[derive(Debug, Default)]
pub struct transaction_service;

#[tonic::async_trait]
impl TransactionService for transaction_service {
    async fn request_money(
        &self,
        request: Request<RequestMoneyRequest>,
    ) -> Result<Response<RequestMoneyResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn confirm_request(
        &self,
        request: Request<ConfirmRequestRequest>,
    ) -> Result<Response<ConfirmRequestResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn send_payment(
        &self,
        request: Request<SendPaymentRequest>,
    ) -> Result<Response<SendPaymentResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = transaction_service::default();

    println!("🚀 TransactionService listening on {}", addr);

    Server::builder()
        .add_service(TransactionServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
