use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use k256::Secp256k1;
use risc0_zkvm::Receipt;
use std::fs::File;
use hex;
use bincode;
use methods::ACROPOLIS_ID;
use risc0_types::CircuitOutputs;
use k256::ecdsa::Signature;
use std::collections::HashMap;

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

pub fn verify_receipt_vec(receipts: Vec<Receipt>, gov_pub_key: String) -> HashMap<String, u64>{
    let mut identities: Vec<Signature> = Vec::new();
    let mut malicious_identities: Vec<Signature> = Vec::new();
    let mut valid_votes: Vec<CircuitOutputs> = Vec::new();
    for receipt in receipts{
        let journal: CircuitOutputs = receipt.journal.decode().expect("Failed to decode journal");
        let journal_gov_pub: String = hex::encode(&journal.government_public_key);
        let voter_identity: Signature = journal.public_identity;
        if journal_gov_pub != gov_pub_key{
            continue;
        };
        if identities.contains(&voter_identity){
            if !malicious_identities.contains(&voter_identity){
                malicious_identities.push(voter_identity);
            };
        };
        if !identities.contains(&voter_identity){
            identities.push(voter_identity);
        }
        // verify the risc0 Receipt
        match receipt.verify(ACROPOLIS_ID){
            Ok(_) => {
                if !malicious_identities.contains(&voter_identity){
                    valid_votes.push(journal.clone());
                }
            },
            Err(_) => {
                eprintln!("Invalid proof: {:?}", &journal)
            }
        }
    };
    // count votes and return results
    let mut votes: HashMap<String, u64> = HashMap::new();
    for vote in valid_votes{
        if !votes.contains_key(&vote.choice){
            votes.insert(vote.choice.clone(), 1u64);
        }
        else{
            let mut current_votes = *votes.get(&vote.choice).expect("Option does not exist");
            current_votes += 1;
            votes.insert(vote.choice, current_votes);
        }
    }
    for result in &votes{
        println!("Candidate {:?} has received {:?} votes", result.0, result.1)
    }
    votes
}
