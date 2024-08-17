use alloy::contract::ContractInstance;
use alloy::network::Ethereum;
use alloy::primitives::{address, Address};
use alloy::providers::ProviderBuilder;
use alloy::sol;
use alloy::transports::http::{Client, Http};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use girlfriend_lib::Message;
use sp1_sdk::{ProverClient, SP1Stdin};

pub const GIRLFRIEND_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");
pub const RPC_URL: &str = "http://localhost:8545";
pub const CONTRACT_ADDRESS: Address = address!("5FbDB2315678afecb367f032d93F642f64180aa3");
pub const ABI: &str = include_str!("../abi.json");

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(prove));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


#[derive(serde::Deserialize)]
struct ProveRequest(Vec<Message>);


#[derive(serde::Serialize)]
struct ProveResponse {
    pub tx: String,
}

async fn prove(Json(payload): Json<ProveRequest>) -> (StatusCode, Json<ProveResponse>) {
    let mut result = tokio::task::spawn_blocking(move || -> Result<_, anyhow::Error> {
        let prover_client = ProverClient::new();
        let mut stdin = SP1Stdin::new();
        stdin.write(&serde_json::to_vec(&payload.0)?);
        let (pk, vk) = prover_client.setup(GIRLFRIEND_ELF);
        let proof = prover_client.prove(&pk, stdin).run()?;
        Ok(proof)
    }).await;

    if result.is_err() {
        return (StatusCode::BAD_REQUEST, Json(ProveResponse { tx: "".to_string() }));
    }
    let result = result.unwrap();

    let eth_client = ProviderBuilder::new().on_http(RPC_URL.parse().unwrap());
    let contract: ContractInstance<Http<Client>, _, Ethereum> =
            ContractInstance::new(CONTRACT_ADDRESS, provider.clone(), Interface::new());

    
    todo!()
}
