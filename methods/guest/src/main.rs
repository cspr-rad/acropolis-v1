use risc0_zkvm::guest::env;
use risc0_types::{CircuitInputs, CircuitOutputs};
use k256::{
    ecdsa::{signature::{Signer, Verifier}, Signature, SigningKey, VerifyingKey}, EncodedPoint
};
fn main() {
    let circuit_inputs: CircuitInputs = env::read();
    let choice: String = circuit_inputs.choice;
    let user_public_key: VerifyingKey = VerifyingKey::from_encoded_point(&EncodedPoint::from_bytes(&circuit_inputs.user_public_key).unwrap()).unwrap();
    let government_public_key: VerifyingKey = VerifyingKey::from_encoded_point(&EncodedPoint::from_bytes(&circuit_inputs.government_public_key).unwrap()).unwrap();

    user_public_key.verify(&circuit_inputs.government_public_key, &Signature::from_slice(&circuit_inputs.session_signature).unwrap()).expect("Failed to verify session signature");
    government_public_key.verify(&circuit_inputs.user_public_key, &Signature::from_slice(&circuit_inputs.public_identity).unwrap()).expect("Failed to verify public identity");

    let output: CircuitOutputs = CircuitOutputs{
        choice: choice,
        government_public_key: circuit_inputs.government_public_key,
        public_identity: circuit_inputs.public_identity
    };
    env::commit(&output);
}