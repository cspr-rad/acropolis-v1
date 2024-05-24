use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint, PublicKey,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CircuitInputs {
    pub choice: String,
    pub user_public_key: Vec<u8>,
    pub user_secret_key: Vec<u8>,
    pub government_public_key: Vec<u8>,
    pub public_identity: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct CircuitOutputs {
    pub choice: String,
    pub government_public_key: Vec<u8>,
    pub public_identity: Vec<u8>,
}
