use notification_stubs::{
    notification_handler_client::NotificationHandlerClient, SendNotificationRequest,
};
use std::env;
use std::sync::Arc;
use std::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};
use transaction_stubs::transaction_handler_server::{TransactionHandler, TransactionHandlerServer};
use transaction_stubs::{
    ConfirmRequestRequest, ConfirmRequestResponse, RejectRequestRequest, RejectRequestResponse,
    RequestMoneyRequest, RequestMoneyResponse, SendPaymentRequest, SendPaymentResponse,
};
use wallet_stubs::wallet_client::WalletClient as WalletHandlerClient;

pub mod transaction_stubs {
    tonic::include_proto!("dubpay.transaction");
}
pub mod notification_stubs {
    tonic::include_proto!("dubpay.notification");
}
pub mod wallet_stubs {
    tonic::include_proto!("dubpay.wallet");
}

#[derive(Debug)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Rejected,
}

impl Default for TransactionStatus {
    fn default() -> Self {
        TransactionStatus::Pending
    }
}

#[derive(Debug, Default)]
pub struct Transaction {
    id: String,
    from: String,
    to: String,
    amount: f64,
    public: bool,
    description: String,
    status: TransactionStatus,
}

#[derive(Debug, Default)]
pub struct TransactionService {
    transactions: Arc<RwLock<Transaction>>,
}

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
            "Requesting transfer of funds from from {} to {} for {}, public: {}, description: {}",
            from, to, amount, public, description
        );

        let mut wallet_handler_client = WalletHandlerClient::connect("http://wallet-service:50051")
            .await
            .unwrap();

        let transfer_request = tonic::Request::new(wallet_stubs::TransferFundsRequest {
            sender_id: from.clone(),
            receiver_id: to.clone(),
            amount,
            description,
        });

        let _ = wallet_handler_client.transfer_funds(transfer_request).await;

        let mut notification_handler_client =
            NotificationHandlerClient::connect("http://notification-service:50051")
                .await
                .unwrap();

        println! {
            "Requesting notification service to send notification to {}",
            to
        };

        let notification_request = tonic::Request::new(SendNotificationRequest {
            user_id: to.clone(),
            message: format!("You received a payment of {} from {}", amount, from),
            timestamp: Some(prost_types::Timestamp {
                seconds: 0,
                nanos: 0,
            }),
        });

        let _ = notification_handler_client
            .send_notification(notification_request)
            .await;

        let notification_request = tonic::Request::new(SendNotificationRequest {
            user_id: from.clone(),
            message: format!("You sent a payment of {} from {}", amount, to),
            timestamp: Some(prost_types::Timestamp {
                seconds: 0,
                nanos: 0,
            }),
        });

        let _ = notification_handler_client
            .send_notification(notification_request)
            .await;

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
    let service = TransactionService {
        transactions: Arc::new(RwLock::new(Transaction::default())),
    };

    println!("🚀 TransactionService listening on {}", addr);

    Server::builder()
        .add_service(TransactionHandlerServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
