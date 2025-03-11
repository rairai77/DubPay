use std::env;

use tonic::{transport::Server, Request, Response, Status};

use dubpay::wallet_server::{Wallet, WalletServer};
use dubpay::{
    CreateWalletRequest, CreateWalletResponse, DepositFundsRequest, DepositFundsResponse,
    GetBalanceRequest, GetBalanceResponse, GetTransactionHistoryRequest,
    GetTransactionHistoryResponse, TransferFundsRequest, TransferFundsResponse,
    WithdrawFundsRequest, WithdrawFundsResponse,
};

pub mod dubpay {
    tonic::include_proto!("dubpay.wallet");
}

#[derive(Debug, Default)]
pub struct WalletService;

#[tonic::async_trait]
impl Wallet for WalletService {
    async fn create_wallet(
        &self,
        _request: Request<CreateWalletRequest>,
    ) -> Result<Response<CreateWalletResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_balance(
        &self,
        _request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
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
        _request: Request<DepositFundsRequest>,
    ) -> Result<Response<DepositFundsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn withdraw_funds(
        &self,
        _request: Request<WithdrawFundsRequest>,
    ) -> Result<Response<WithdrawFundsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("SERVICE_PORT").unwrap_or_else(|_| "50053".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let service = WalletService;

    println!("Wallet Service listening on {}", addr);

    Server::builder()
        .add_service(WalletServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
