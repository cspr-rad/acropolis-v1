use bincode;
use hex;
use methods::ACROPOLIS_ID;
use risc0_zkvm::Receipt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn parse_receipts_file(path: PathBuf) -> Vec<Receipt> {
    let file = File::open(path).expect("Failed to read receipts file");
    let reader = BufReader::new(file);
    let mut result: Vec<Receipt> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(hex_encoded_receipt) => {
                let receipt_bytes: Vec<u8> =
                    hex::decode(hex_encoded_receipt).expect("Failed to decode receipt hex");
                let receipt: Receipt =
                    bincode::deserialize(&receipt_bytes).expect("Failed to deserialize receipt");
                result.push(receipt);
            }

            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    result
}

pub fn serialize_receipt(receipt: Receipt) -> Vec<u8> {
    bincode::serialize(&receipt).expect("Failed to serialize receipt")
}

pub fn verify_receipt_vec(receipts: Vec<Receipt>) {
    for receipt in receipts {
        receipt
            .verify(ACROPOLIS_ID)
            .expect("Failed to verify proof");
    }
}
