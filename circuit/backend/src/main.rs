use std::io::Write;

use alloy::contract::{ContractInstance, Interface};
use alloy::dyn_abi::DynSolValue;
use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::{address, Address, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::{Client, Http};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use girlfriend_lib::Message;
use sp1_sdk::{ProverClient, SP1Stdin};

use tower_http::trace::TraceLayer;

pub const GIRLFRIEND_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");
pub const RPC_URL: &str = "https://sepolia.drpc.org";
pub const CONTRACT_ADDRESS: Address = address!("1Ab75789b588f1EdB8286d05bD482e0E13FcA248");
pub const ABI: &str = include_str!("../abi.json");

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    let app = Router::new().route("/", post(prove)).layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Deserialize, Debug)]
struct ProveRequest {
    address: String,
    proofs: Vec<String>,
}

#[derive(serde::Serialize)]
struct ProveResponse {
    pub tx: String,
}

async fn prove(Json(payload_full): Json<ProveRequest>) -> (StatusCode, Json<ProveResponse>) {
    println!("Payload: {:?}", payload_full);
    let address = payload_full.address.parse::<Address>().unwrap();
    tracing::info!("Address: {:?}", address);
    let payload = payload_full
        .proofs
        .into_iter()
        .map(|x| serde_json::from_str::<Message>(&x).unwrap())
        .collect::<Vec<_>>();
    let payload_log = serde_json::to_string(&payload).unwrap();
    println!("Payload: {}", payload_log);
    let (public_inputs, proof_bytes) = if std::fs::metadata("proof").is_ok() {
        let proof = std::fs::read("proof").unwrap();
        let public_inputs_2 = std::fs::read("public_inputs").unwrap();
        let proof_bytes = DynSolValue::Bytes(proof.clone());
        let public_inputs = DynSolValue::Bytes(public_inputs_2.to_vec());
        (public_inputs, proof_bytes)
    } else {
        let prover_client = ProverClient::new();
        let result = tokio::task::spawn_blocking(move || -> Result<_, anyhow::Error> {
            let mut stdin = SP1Stdin::new();
            stdin.write(&serde_json::to_vec(&payload)?);
            let (pk, _vk) = prover_client.setup(GIRLFRIEND_ELF);
            let proof = prover_client.prove(&pk, stdin).plonk().run()?;
            Ok(proof)
        });
        let result = match result.await {
            Ok(Ok(result)) => result,
            Ok(Err(err)) => {
                tracing::error!("Error: {:?}", err);
                return (StatusCode::BAD_REQUEST, Json(ProveResponse { tx: "".to_string() }));
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ProveResponse { tx: "".to_string() }),
                )
            }
        };

        let proof = result.bytes();
        let public_inputs_2 = result.public_values.as_slice();
        let proof_bytes = DynSolValue::Bytes(proof.clone());
        let public_inputs = DynSolValue::Bytes(public_inputs_2.to_vec());

        let file = std::fs::File::create("proof").unwrap();
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(&proof).unwrap();
        writer.flush().unwrap();
        let file = std::fs::File::create("public_inputs").unwrap();
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(public_inputs_2).unwrap();
        writer.flush().unwrap();

        (public_inputs, proof_bytes)
    };

    // let signer: PrivateKeySigner = "<PRIVATE_KEY>".parse().expect("should parse private key");
    let signer: PrivateKeySigner =
        std::env::var("PRIVATE_KEY").unwrap().parse().expect("should parse private key");
    let wallet = EthereumWallet::from(signer);
    let eth_client = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(RPC_URL.parse().unwrap());
    let contract: ContractInstance<Http<Client>, _, Ethereum> = ContractInstance::new(
        CONTRACT_ADDRESS,
        eth_client,
        Interface::new(serde_json::from_str(ABI).unwrap()),
    );

    let tx = match contract.function(
        "newGirlfriend",
        &[public_inputs, proof_bytes, address.into(), U256::from(50).into()],
    ) {
        Ok(tx) => tx,
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ProveResponse { tx: "".to_string() }));
        }
    };
    println!("Transaction: {:?}", tx);

    match tx.send().await {
        Ok(tx) => (
            StatusCode::OK,
            Json(ProveResponse {
                tx: tx.tx_hash().to_string(),
            }),
        ),
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ProveResponse { tx: "".to_string() }))
        }
    }
}
