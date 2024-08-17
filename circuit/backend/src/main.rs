use alloy::contract::{ContractInstance, Interface};
use alloy::dyn_abi::DynSolValue;
use alloy::network::Ethereum;
use alloy::primitives::{address, Address};
use alloy::providers::ProviderBuilder;
use alloy::transports::http::{Client, Http};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use girlfriend_lib::Message;
use sp1_sdk::{ProverClient, SP1Stdin};

pub const GIRLFRIEND_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");
pub const RPC_URL: &str = "https://sepolia.drpc.org";
pub const CONTRACT_ADDRESS: Address = address!("95468FA91b7DC836a0d0dE781835dA80B2e52d39");
pub const ABI: &str = include_str!("../abi.json");

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(prove));
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
    let result = tokio::task::spawn_blocking(move || -> Result<_, anyhow::Error> {
        let prover_client = ProverClient::new();
        let mut stdin = SP1Stdin::new();
        stdin.write(&serde_json::to_vec(&payload.0)?);
        let (pk, _vk) = prover_client.setup(GIRLFRIEND_ELF);
        let proof = prover_client.prove(&pk, stdin).plonk().run()?;
        Ok(proof)
    });
    let result = match result.await {
        Ok(Ok(result)) => result,
        Ok(Err(_)) => return (StatusCode::BAD_REQUEST, Json(ProveResponse { tx: "".to_string() })),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ProveResponse { tx: "".to_string() }))
        }
    };

    let eth_client = ProviderBuilder::new().on_http(RPC_URL.parse().unwrap());
    let contract: ContractInstance<Http<Client>, _, Ethereum> = ContractInstance::new(
        CONTRACT_ADDRESS,
        eth_client,
        Interface::new(serde_json::from_str(ABI).unwrap()),
    );

    let proof = result.bytes();
    let public_inputs = result.public_values.as_slice();
    let proof_bytes = DynSolValue::Bytes(proof);
    let public_inputs = DynSolValue::Bytes(public_inputs.to_vec());

    let tx = match contract.function("newGirlfriend", &[public_inputs, proof_bytes]) {
        Ok(tx) => tx,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ProveResponse { tx: "".to_string() }))
        }
    };

    match tx.send().await {
        Ok(tx) => (
            StatusCode::OK,
            Json(ProveResponse {
                tx: tx.tx_hash().to_string(),
            }),
        ),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ProveResponse { tx: "".to_string() })),
    }
}
