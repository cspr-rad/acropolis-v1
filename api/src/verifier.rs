use risc0_zkvm::Receipt;
use risc0_types::CircuitOutputs;
use k256::{
    ecdsa::{signature::{Signer, Verifier}, Signature, SigningKey, VerifyingKey}, EncodedPoint
};
pub fn verify_receipt(receipt: Receipt, program_id: [u32;8]) -> CircuitOutputs{
    receipt.verify(program_id).expect("Failed to verify proof");
    receipt.journal.decode().expect("Failed to extract public journal from receipt")
}

pub fn verify_receipt_against(receipt: Receipt, program_id: [u32;8], government_public_key: VerifyingKey){
    let journal = verify_receipt(receipt, program_id);
    assert_eq!(VerifyingKey::from_encoded_point(&EncodedPoint::from_bytes(&journal.government_public_key).expect("Failed to parse as encoded point")).expect("Failed to parse as verifying key"), government_public_key);
}

// todo: perform an additional check that journal.public_identity is in gov_sigs: Vec<todo!("sigs")>
// and that the proof is unique