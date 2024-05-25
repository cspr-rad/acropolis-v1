use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint,
};
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
    // receipt journal contains all info about the vote e.g. option, government identity, ...
    // we want to store this so that an external observer can verify all proofs independently
    pub receipts: Vec<Receipt>,
    // this is where we store votes that have been verified
    // their government_public_key should match the gov_key of this Election
    pub receipt_journals_decoded: Vec<CircuitOutputs>,
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
