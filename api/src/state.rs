use crate::{verifier, CONFIG};
use k256::ecdsa::{Signature, VerifyingKey};
use risc0_types::CircuitOutputs;
use risc0_zkvm::Receipt;
use std::collections::HashMap;
use std::fs;
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
        let path_to_resources = CONFIG.server.resources_path.clone();
        let path_to_election_1 = path_to_resources.join("election-1");
        let path_to_election_2 = path_to_resources.join("election-2");
        fs::create_dir_all(&path_to_election_1).expect("Failed to create resources directory");
        fs::create_dir_all(&path_to_election_2).expect("Failed to create resources directory");
        // create 5 users for each election
        // create 1 government account for each election
        // authorize all 5 users for each election
        Arc::new(Mutex::new(AppState {
            state: MockBlockChainState::default(),
        }))
    }

    pub fn process_receipt(&mut self, receipt: crate::verifier::Receipt) {
        let outputs: CircuitOutputs = verifier::verify_receipt(receipt.clone());
        self.state
            .elections
            .iter_mut()
            .filter(|election| election.gov_key == outputs.deserialized_government_public_key())
            .for_each(|election| {
                election
                    .receipts
                    .insert(outputs.public_identity.to_bytes().to_vec(), receipt.clone());
                election
                    .receipt_journals_decoded
                    .insert(outputs.public_identity.to_bytes().to_vec(), outputs.clone());
            });
    }

    // pass the gov_key of the election to this function to fetch the metadata of all votes that have previously been verified
    // and are associated with that exact election
    pub fn fetch_census_votes(&self, gov_key: VerifyingKey) -> Option<Vec<CircuitOutputs>> {
        let verified_votes: Vec<CircuitOutputs> = self
            .state
            .elections
            .iter()
            .filter(|election| election.gov_key == gov_key)
            .flat_map(|election| election.receipt_journals_decoded.values().cloned())
            .collect();
        if verified_votes.is_empty() {
            None
        } else {
            Some(verified_votes)
        }
    }
}
