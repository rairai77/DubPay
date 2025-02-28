use dubpay::{
    fraud_detection_handler_server::{FraudDetectionHandler, FraudDetectionHandlerServer},
    AnalyzeHighValueTransactionRequest, AnalyzeHighValueTransactionResponse,
    AnalyzeTransactionRequest, AnalyzeTransactionResponse,
};
use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod dubpay {
    tonic::include_proto!("dubpay.fraud_detection");
}

#[derive(Debug, Default)]
pub struct FraudDetectionService;

#[tonic::async_trait]
impl FraudDetectionHandler for FraudDetectionService {
    async fn analyze_transaction(
        &self,
        _request: Request<AnalyzeTransactionRequest>,
    ) -> Result<Response<AnalyzeTransactionResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn analyze_high_value_transaction(
        &self,
        _request: Request<AnalyzeHighValueTransactionRequest>,
    ) -> Result<Response<AnalyzeHighValueTransactionResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("SERVICE_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("[::1]:{}", port).parse()?;
    let service = FraudDetectionService;

    println!("🚀 Fraud Detection Service listening on {}", addr);

    Server::builder()
        .add_service(FraudDetectionHandlerServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
