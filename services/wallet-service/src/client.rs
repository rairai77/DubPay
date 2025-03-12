use std::env;

use tonic::{transport::Server, Request, Response, Status};

use dubpay::wallet_server::{Wallet, WalletServer};
use dubpay::{
    notification_handler_client, wallet_client, CreateWalletRequest, CreateWalletResponse, DepositFundsRequest, DepositFundsResponse, GetBalanceRequest, GetBalanceResponse, GetTransactionHistoryRequest, GetTransactionHistoryResponse, TransferFundsRequest, TransferFundsResponse, WithdrawFundsRequest, WithdrawFundsResponse
};

pub mod dubpay {
    tonic::include_proto!("dubpay.wallet");
    tonic::include_proto!("dubpay.notification");

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let mut client = wallet_client::WalletClient::connect("http://[::1]:50057").await?;

    let request = tonic::Request::new(CreateWalletRequest {
        user_id: "1234".to_string(),
    });

    let response = client.create_wallet(request).await?;

    Ok(())
}


