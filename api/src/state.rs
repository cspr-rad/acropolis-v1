use crate::verifier;
use k256::ecdsa::{Signature, VerifyingKey};
use risc0_types::CircuitOutputs;
use risc0_zkvm::Receipt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type StateType = Arc<Mutex<AppState>>;

extern crate alloc;
use alloc::collections::BTreeMap;

type PublicIdentity = Vec<u8>;
type GovernmentPublicKey = VerifyingKey;

#[derive(Default, Clone)]
pub struct MockBlockChainState {
    elections: Vec<Election>,
}

#[derive(Clone)]
pub struct Election {
    pub gov_key: GovernmentPublicKey,
    pub gov_sigs: Vec<Signature>,
    pub options: Vec<String>,
    // take the public_identity from the CircuitOutputs and insert <public_identity, receipt> into this HashSet
    // every key in the HashSet will be unique => every public identity can only vote once.
    pub receipts: HashMap<PublicIdentity, Receipt>,
    // this is where we store votes that have been verified
    // their government_public_key should match the gov_key of this Election
    // this is what will be returned to the front-end.
    pub receipt_journals_decoded: BTreeMap<PublicIdentity, CircuitOutputs>,
}

#[derive(Clone)]
pub struct AppState {
    state: MockBlockChainState,
}

impl AppState {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(AppState {
            state: MockBlockChainState::default(),
        }))
    }

    pub fn process_receipt(&mut self, receipt: crate::verifier::Receipt) {
        let outputs: CircuitOutputs = verifier::verify_receipt(receipt.clone());
        for election in &mut self.state.elections {
            if election.gov_key == outputs.deserialized_government_public_key() {
                election
                    .receipts
                    .insert(outputs.public_identity.to_bytes().to_vec(), receipt.clone());
                election
                    .receipt_journals_decoded
                    .insert(outputs.public_identity.to_bytes().to_vec(), outputs.clone());
            }
        }
    }
}
