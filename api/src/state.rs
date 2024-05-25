use crate::{verifier, CONFIG};
use k256::{ecdsa::{Signature, VerifyingKey, SigningKey}, EncodedPoint};
use risc0_types::CircuitOutputs;
use risc0_zkvm::Receipt;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use acropolis::{Cli, run, Command, VerifiedUser};
use std::path::PathBuf;
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
        if fs::metadata(&path_to_election_1).is_ok() {
            fs::remove_dir_all(&path_to_election_1).expect("Failed to delete resources");
        }
        if fs::metadata(&path_to_election_2).is_ok() {
            fs::remove_dir_all(&path_to_election_2).expect("Failed to delete resources");
        }

        fs::create_dir_all(&path_to_election_1).expect("Failed to create resources directory for election 1");
        fs::create_dir_all(&path_to_election_2).expect("Failed to create resources directory for election 2");

        let mut election_1_gov_sigs: Vec<Signature> = Vec::new();
        let mut election_2_gov_sigs: Vec<Signature> = Vec::new();

        let election_1_options: Vec<String> = vec!["matthew".to_string(), "marijan".to_string()];
        let election_2_options: Vec<String> = vec!["rome".to_string(), "jonas".to_string()];
        // create 5 users for each election
        // create 1 government account for each election
        // authorize all 5 users for each election and register them
        let usernames: Vec<String> = vec!["user-1".to_string(), "user-2".to_string(), "user-3".to_string(), "user-4".to_string(), "user-5".to_string()];
        let create_user_election_one_command = Command::GenerateKeyPair { out_path: path_to_election_1.clone(), user_name: Some("admin".to_string()) };
        run(Cli { command: create_user_election_one_command });
        let create_user_election_two_command = Command::GenerateKeyPair { out_path: path_to_election_2.clone(), user_name: Some("admin".to_string()) };
        run(Cli { command: create_user_election_two_command });
    
        for username in usernames{
            // create current user for election 1
            let create_user_election_one_command = Command::GenerateKeyPair { out_path: path_to_election_1.clone(), user_name: Some(username.clone()) };
            run(Cli { command: create_user_election_one_command });
            // create current user for election 2
            let create_user_election_two_command = Command::GenerateKeyPair { out_path: path_to_election_2.clone(), user_name: Some(username.clone()) };
            run(Cli { command: create_user_election_two_command });
            // create gov id for user for election 1
            let create_gov_id_election_one_command = Command::IssueIdentity { issuer_skey_path: path_to_election_1.join("admin".to_string()).join("secret_key"), user_pkey_path: path_to_election_1.join(username.clone()).join("public_key") };
            run(Cli {command: create_gov_id_election_one_command});
            // create gov id for user for election 2
            let create_gov_id_election_two_command = Command::IssueIdentity { issuer_skey_path: path_to_election_2.join("admin".to_string()).join("secret_key"), user_pkey_path: path_to_election_2.join(username.clone()).join("public_key") };
            run(Cli {command: create_gov_id_election_two_command});
            // add public identity of user to election 1 vec
            let verified_user_e1: VerifiedUser = serde_json::from_str(
                &fs::read_to_string(path_to_election_1.join(username.clone()).join("public_identity")).expect(""),
            )
            .expect("");
            let public_identity_e1 = Signature::from_slice(&verified_user_e1.public_identity).expect("");
            election_1_gov_sigs.push(public_identity_e1);
            // add public identity of user to election 2 vec
            let verified_user_e2: VerifiedUser = serde_json::from_str(
                &fs::read_to_string(path_to_election_2.join(username.clone()).join("public_identity")).expect(""),
            )
            .expect("");
            let public_identity_e2 = Signature::from_slice(&verified_user_e2.public_identity).expect("");
            election_2_gov_sigs.push(public_identity_e2);
        }
        let admin_election_1_public_key =
        *SigningKey::from_slice(&fs::read(path_to_election_1.join("admin").join("secret_key")).expect(""))
            .expect("").verifying_key();

        let admin_election_2_public_key =
        *SigningKey::from_slice(&fs::read(path_to_election_2.join("admin").join("secret_key")).expect(""))
            .expect("").verifying_key();

        let elections: Vec<Election> = vec![
            Election{
                gov_key: admin_election_1_public_key,
                gov_sigs: election_1_gov_sigs,
                options: election_1_options,
                receipts: HashMap::new(),
                receipt_journals_decoded: BTreeMap::new()
            },
            Election{
                gov_key: admin_election_2_public_key,
                gov_sigs: election_2_gov_sigs,
                options: election_2_options,
                receipts: HashMap::new(),
                receipt_journals_decoded: BTreeMap::new()
            }
        ];
        Arc::new(Mutex::new(AppState {
            state: MockBlockChainState{
                elections
            },
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
