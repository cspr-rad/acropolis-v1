use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint,
};
use std::collections::HashSet;
use risc0_types::CircuitOutputs;
use risc0_zkvm::Receipt;

#[derive(Default, Clone)]
pub struct MockBlockChainState {
    pub elections: Vec<Election>,
}

#[derive(Clone)]
pub struct Election {
    pub gov_key: VerifyingKey,
    pub gov_sigs: Vec<Signature>,
    pub options: Vec<String>,
    // take the public_identity from the CircuitOutputs and insert <public_identity, receipt> into this HashSet
    // every key in the HashSet will be unique => every public identity can only vote once.
    pub receipts: HashSet<Signature, Receipt>,
    // this is where we store votes that have been verified
    // their government_public_key should match the gov_key of this Election
    // this is what will be returned to the front-end.
    pub receipt_journals_decoded: HashSet<Signature, CircuitOutputs>,
}

#[derive(Clone)]
pub struct AppState {
    pub state: MockBlockChainState,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            state: MockBlockChainState::default(),
        }
    }
}
