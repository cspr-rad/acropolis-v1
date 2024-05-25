mod prover;
use clap::{Parser, Subcommand};
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
    EncodedPoint,
};
use rand_core::OsRng;
use reqwest::blocking::Client;
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
    },
    GenerateKeyPair {
        #[arg(short, long)]
        out_path: PathBuf,
        #[arg(short, long)]
        user_name: Option<String>,
    },
    IssueIdentity {
        #[arg(short, long)]
        issuer_skey_path: PathBuf,
        #[arg(short, long)]
        user_pkey_path: PathBuf,
    },
}

#[derive(Serialize, Deserialize)]
struct VerifiedUser {
    government_public_key: Vec<u8>,
    public_identity: Vec<u8>,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Vote { user_id_path, vote } => {
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
            let receipt = prover::prove(
                &vote,
                &user_secret_key,
                &government_public_key,
                &public_identity,
            );
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
            issuer_skey_path,
            user_pkey_path,
        } => {
            let issuer_skey =
                SigningKey::from_slice(&fs::read(issuer_skey_path).expect("")).expect("");
            let public_identity: Signature =
                issuer_skey.sign(&fs::read(&user_pkey_path).expect(""));
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
    }
}
