use serde::{Deserialize, Serialize};
use k256::{ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey}, EncodedPoint};

#[derive(Serialize, Deserialize)]
pub struct CircuitInputs {
    pub choice: String,
    pub user_public_key: Vec<u8>,
    pub session_signature: Signature,
    pub government_public_key: Vec<u8>,
    pub public_identity: Signature,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CircuitOutputs {
    pub choice: String,
    pub government_public_key: Vec<u8>,
    pub public_identity: Signature,
}

impl CircuitOutputs{
    pub fn deserialized_government_public_key(&self) -> VerifyingKey{
        VerifyingKey::from_encoded_point(&EncodedPoint::from_bytes(&self.government_public_key).unwrap()).unwrap()
    }
}