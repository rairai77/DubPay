use dubpay::transaction_handler_server::{TransactionHandler, TransactionHandlerServer};
use dubpay::{
    ConfirmRequestRequest, ConfirmRequestResponse, RequestMoneyRequest, RequestMoneyResponse,
    SendPaymentRequest, SendPaymentResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod dubpay {
    tonic::include_proto!("dubpay.transaction");
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

    async fn send_payment(
        &self,
        _request: Request<SendPaymentRequest>,
    ) -> Result<Response<SendPaymentResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = TransactionService;

    println!("🚀 TransactionService listening on {}", addr);

    Server::builder()
        .add_service(TransactionHandlerServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
