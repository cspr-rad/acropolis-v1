// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::ACROPOLIS_ELF;
use k256::{ecdsa::{
        signature::Signer,
        Signature, SigningKey, VerifyingKey,
    }};
use risc0_types::CircuitInputs;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

pub fn prove(
    choice: String,
    user_secret_key: SigningKey,
    government_public_key: VerifyingKey,
    public_identity: Signature,
) -> Receipt {
    let user_public_key_serialized: Vec<u8> = user_secret_key
        .verifying_key()
        .clone()
        .to_encoded_point(true)
        .to_bytes()
        .to_vec();
    let government_public_key_serialized: Vec<u8> = government_public_key.to_encoded_point(true).to_bytes().to_vec();
    let unique_session_signature: Signature = user_secret_key.sign(&government_public_key.to_encoded_point(true).to_bytes().to_vec());
    let circuit_inputs: CircuitInputs = CircuitInputs {
        choice: choice,
        user_public_key: user_public_key_serialized,
        session_signature: unique_session_signature,
        government_public_key: government_public_key_serialized,
        public_identity: public_identity,
    };

    let env = ExecutorEnv::builder()
        .write(&circuit_inputs)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    prover.prove(env, ACROPOLIS_ELF).unwrap()
}

#[test]
fn generate_proof() {
    use rand_core::OsRng;
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key_bytes = signing_key
        .verifying_key()
        .to_encoded_point(true)
        .to_bytes()
        .to_vec();
    let government_signing_key: SigningKey = SigningKey::random(&mut OsRng);
    let public_identity: Signature = government_signing_key.sign(&verifying_key_bytes);
    let choice: String = "42".to_string();
    prove(
        choice,
        signing_key,
        *government_signing_key.verifying_key(),
        public_identity,
    );
}
