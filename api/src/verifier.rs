use risc0_zkvm::Receipt;
use risc0_types::CircuitOutputs;

pub fn verify_receipt(receipt: Receipt, program_id: [u32;8]) -> CircuitOutputs{
    receipt.verify(program_id).expect("Failed to verify proof");
    receipt.journal.decode().expect("Failed to extract public journal from receipt")
}