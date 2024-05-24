use risc0_zkvm::Receipt;
use risc0_types::CircuitOutputs;
use k256::{
    ecdsa::{signature::{Signer, Verifier}, Signature, SigningKey, VerifyingKey}, EncodedPoint
};
pub fn verify_receipt(receipt: Receipt, program_id: [u32;8]) -> CircuitOutputs{
    receipt.verify(program_id).expect("Failed to verify proof");
    receipt.journal.decode().expect("Failed to extract public journal from receipt")
}

// todo: perform an additional check that journal.public_identity is in gov_sigs: Vec<todo!("sigs")>
// and that the proof is unique