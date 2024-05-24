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
struct MockBlockChainState {
    elections: Vec<Election>,
}

#[derive(Clone)]
struct Election {
    gov_key: VerifyingKey,
    gov_sigs: Vec<Signature>,
    options: Vec<String>,
    // receipt journal contains all info about the vote e.g. option, government identity, ...
    receipts: Vec<Receipt>,
    // this is where we store votes that have been verified
    // their government_public_key should match the gov_key of this Election
    receipt_journals_decoded: Vec<CircuitOutputs>,
}

#[derive(Clone)]
pub struct AppState {
    state: MockBlockChainState,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            state: MockBlockChainState::default(),
        }
    }
}
