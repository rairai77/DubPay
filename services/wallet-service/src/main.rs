use std::env;

use tonic::{transport::Server, Request, Response, Status};

use dubpay::wallet_server::{Wallet, WalletServer};
use dubpay::{
    notification_handler_client, CreateWalletRequest, CreateWalletResponse, DepositFundsRequest, DepositFundsResponse,
    GetBalanceRequest, GetBalanceResponse, GetTransactionHistoryRequest,
    GetTransactionHistoryResponse, TransferFundsRequest, TransferFundsResponse,
    WithdrawFundsRequest, WithdrawFundsResponse,
};

pub mod dubpay {
    tonic::include_proto!("dubpay.wallet");
    tonic::include_proto!("dubpay.notification");

}

#[derive(Debug, Default)]
pub struct WalletService;

#[tonic::async_trait]
impl Wallet for WalletService {
    async fn create_wallet(
        &self,
        request: Request<CreateWalletRequest>,
    ) -> Result<Response<CreateWalletResponse>, Status> {
        let request_content = request.into_inner();
        let userId = request_content.user_id;
       
        println!(
            "Creating a wallet for user with ID {}",
            userId
        );

        let mut notification_handler_client =
            notification_handler_client::NotificationHandlerClient::connect(
                "http://notification-service:50051",
            )
            .await
            .unwrap();

        let notification_request = tonic::Request::new(dubpay::SendNotificationRequest {
            user_id: userId.to_string(),
            message: format!("You have created a wallet for {}", userId.to_string()),
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

        return Ok(Response::new(CreateWalletResponse {
            wallet_id: "1234".to_string(),
            user_id: userId.to_string(),
            balance: 0.0
        }));

   }

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let request_content = request.into_inner();
        let userId = request_content.user_id;
      
       
        println!(
            "Getting balance for user with ID {}",
            userId
        );

        let mut notification_handler_client =
            notification_handler_client::NotificationHandlerClient::connect(
                "http://notification-service:50051",
            )
            .await
            .unwrap();

        let notification_request = tonic::Request::new(dubpay::SendNotificationRequest {
            user_id: userId.to_string(),
            message: format!("You have created a wallet for {}", userId.to_string()),
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

        return Ok(Response::new(GetBalanceResponse {
            balance: 0.0
        }));

    }

    async fn transfer_funds(
        &self,
        _request: Request<TransferFundsRequest>,
    ) -> Result<Response<TransferFundsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_transaction_history(
        &self,
        _request: Request<GetTransactionHistoryRequest>,
    ) -> Result<Response<GetTransactionHistoryResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn deposit_funds(
        &self,
        request: Request<DepositFundsRequest>,
    ) -> Result<Response<DepositFundsResponse>, Status> {
        let request_content = request.into_inner();
        let userId = request_content.user_id;
        let amount = request_content.amount;
       
        println!(
            "Depositing {} for {}", amount,
            userId
        );

        let mut notification_handler_client =
            notification_handler_client::NotificationHandlerClient::connect(
                "http://notification-service:50051",
            )
            .await
            .unwrap();

        let notification_request = tonic::Request::new(dubpay::SendNotificationRequest {
            user_id: userId.to_string(),
            message: format!("You have created a wallet for {}", userId.to_string()),
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

        return Ok(Response::new(DepositFundsResponse {
            success: true,
            new_balance: 0.0
        }));

    }

    async fn withdraw_funds(
        &self,
        request: Request<WithdrawFundsRequest>,
    ) -> Result<Response<WithdrawFundsResponse>, Status> {
        let request_content = request.into_inner();
        let userId = request_content.user_id;
        let amount = request_content.amount;
        println!(
            "withdrawing {} for user with ID {}", amount,
            userId
        );

        let mut notification_handler_client =
            notification_handler_client::NotificationHandlerClient::connect(
                "http://notification-service:50051",
            )
            .await
            .unwrap();

        let notification_request = tonic::Request::new(dubpay::SendNotificationRequest {
            user_id: userId.to_string(),
            message: format!("You have created a wallet for {}", userId.to_string()),
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

        return Ok(Response::new(WithdrawFundsResponse {
            success: true,
            new_balance: 0.0
        }));

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("SERVICE_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let service = WalletService;

    println!("Wallet Service listening on {}", addr);

    Server::builder()
        .add_service(WalletServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
