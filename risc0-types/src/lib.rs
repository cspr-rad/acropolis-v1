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
    pub session_signature: Signature,
    pub government_public_key: Vec<u8>,
    pub public_identity: Signature,
}

#[derive(Serialize, Deserialize)]
pub struct CircuitOutputs {
    pub choice: String,
    pub government_public_key: Vec<u8>,
    pub public_identity: Signature,
}
