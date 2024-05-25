mod prover;
use audit_utils::audit_data;
use clap::{Parser, Subcommand};
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
    EncodedPoint,
};
use rand_core::OsRng;
use reqwest::blocking::Client;
use risc0_types::CircuitOutputs;
use risc0_zkvm::Receipt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Vote {
        #[arg(short, long)]
        user_id_path: PathBuf,
        #[arg(short, long)]
        vote: String,
        #[arg(short, long)]
        receipt_out_path: Option<PathBuf>,
        #[cfg(feature = "groth16")]
        #[cfg_attr(feature = "groth16", arg(short, long))]
        groth16_receipt_out_path: Option<PathBuf>,
    },
    GenerateKeyPair {
        #[arg(short, long)]
        out_path: PathBuf,
        #[arg(short, long)]
        user_name: Option<String>,
    },
    IssueIdentity {
        #[arg(short, long)]
        issuer_id_path: PathBuf,
        #[arg(short, long)]
        user_pkey_path: PathBuf,
    },
    Audit {
        #[arg(short, long)]
        audit_file_path: PathBuf,
        #[arg(short, long)]
        gov_key_hex: String,
    },
    #[cfg(feature = "groth16")]
    Groth16Proof {
        #[arg(short, long)]
        receipt_path: PathBuf,
        #[arg(short, long)]
        out_path: PathBuf,
    },
    ExtractElectionId {
        #[arg(short, long)]
        receipt_path: PathBuf,
    },
}

#[derive(Serialize, Deserialize)]
pub struct VerifiedUser {
    pub government_public_key: Vec<u8>,
    pub public_identity: Vec<u8>,
}

pub fn run(cli: Cli) {
    match cli.command {
        Command::Vote {
            user_id_path,
            vote,
            receipt_out_path,
            #[cfg(feature = "groth16")]
            groth16_receipt_out_path,
        } => {
            let user_secret_key =
                SigningKey::from_slice(&fs::read(user_id_path.join("secret_key")).expect(""))
                    .expect("");
            let verified_user: VerifiedUser = serde_json::from_str(
                &fs::read_to_string(user_id_path.join("public_identity")).expect(""),
            )
            .expect("");
            let government_public_key = VerifyingKey::from_encoded_point(
                &EncodedPoint::from_bytes(verified_user.government_public_key).expect(""),
            )
            .expect("");
            let public_identity = Signature::from_slice(&verified_user.public_identity).expect("");

            // generate a regular risc0 proof and submit it to the API server
            let receipt = prover::prove(
                &vote,
                &user_secret_key,
                &government_public_key,
                &public_identity,
            );

            #[cfg(feature = "groth16")]
            if let Some(groth16_receipt_out_path) = groth16_receipt_out_path {
                let groth16_receipt = prover::prove_groth16(&receipt);
                fs::write(
                    groth16_receipt_out_path,
                    format!(
                        "0x{}",
                        hex::encode(bincode::serialize(&groth16_receipt).expect(""))
                    ),
                )
                .expect("");
            }

            if let Some(receipt_out_path) = receipt_out_path {
                let serialized_receipt = bincode::serialize(&receipt).expect("");
                fs::write(receipt_out_path, serialized_receipt).expect("");
            };

            let client: Client = Client::new();
            let response = client
                .post("http://127.0.0.1:8080/submit_receipt")
                .json(&receipt)
                .send()
                .expect("Failed to submit proof to server");
            assert!(response.status().is_success());
        }

        Command::GenerateKeyPair {
            out_path,
            user_name,
        } => {
            let signing_key = SigningKey::random(&mut OsRng);
            let public_key = signing_key.verifying_key();
            let out_path = out_path.join(user_name.unwrap_or("user".to_string()));
            fs::create_dir_all(&out_path).expect("");
            fs::write(out_path.join("secret_key"), signing_key.to_bytes()).expect("");
            fs::write(
                out_path.join("public_key"),
                public_key.to_encoded_point(true).as_bytes(),
            )
            .expect("");
        }
        Command::IssueIdentity {
            issuer_id_path,
            user_pkey_path,
        } => {
            let issuer_skey =
                SigningKey::from_slice(&fs::read(issuer_id_path.join("secret_key")).expect(""))
                    .expect("");

            let mut payload = fs::read(&user_pkey_path).expect("");
            payload.append(&mut fs::read(issuer_id_path.join("public_key")).expect(""));

            let public_identity: Signature = issuer_skey.sign(&payload);

            let mut public_identity_out_path = user_pkey_path.clone();
            public_identity_out_path.set_file_name("public_identity");

            let verified_user = VerifiedUser {
                government_public_key: issuer_skey
                    .verifying_key()
                    .to_encoded_point(true)
                    .to_bytes()
                    .to_vec(),
                public_identity: public_identity.to_bytes().to_vec(),
            };

            fs::write(
                public_identity_out_path,
                serde_json::to_string(&verified_user).expect(""),
            )
            .expect("");
        }
        Command::Audit {
            audit_file_path,
            gov_key_hex,
        } => {
            audit_data(audit_file_path, gov_key_hex);
        }
        #[cfg(feature = "groth16")]
        Command::Groth16Proof {
            receipt_path,
            out_path,
        } => {
            let receipt_bytes = fs::read(receipt_path).expect("");
            let receipt: Receipt = bincode::deserialize(&receipt_bytes).expect("");

            let groth16_receipt = prover::prove_groth16(&receipt);
            fs::write(
                out_path,
                format!(
                    "0x{}",
                    hex::encode(bincode::serialize(&groth16_receipt).expect(""))
                ),
            )
            .expect("");
        }
        Command::ExtractElectionId { receipt_path } => {
            let receipt_bytes = fs::read(receipt_path).expect("");
            let receipt: Receipt = bincode::deserialize(&receipt_bytes).expect("");
            let data: CircuitOutputs = receipt
                .journal
                .decode()
                .expect("Failed to extract public journal from receipt");
            println!("0x{}", hex::encode(data.government_public_key))
        }
    }
}
