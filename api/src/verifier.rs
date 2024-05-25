use risc0_types::CircuitOutputs;
use crate::state::MockBlockChainState;
use risc0_zkvm::Receipt;
use methods::ACROPOLIS_ID;

pub fn verify_receipt(receipt: Receipt) -> CircuitOutputs {
    receipt.verify(ACROPOLIS_ID).expect("Failed to verify proof");
    receipt
        .journal
        .decode()
        .expect("Failed to extract public journal from receipt")
}

// todo: perform an additional check that journal.public_identity is in gov_sigs: Vec<todo!("sigs")>
// and that the proof is unique
