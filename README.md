# Acropolis - a ZKVM enabled voting mechanism

## Authorized Anonymous Voting

Acropolis enables anonymous voting through `Signatures` over `Public Keys`. The cryptographic identity of each user is treated as a secret input and never revealed to the public. Only the `Signature` issued by an authority for a "KYC-ed" user, alongside with the corresponding `Public Key` (one `Public Key` per election that is owned by the authority) are published for each election. An election is defined as a cycle with a fixed set of options to choose from and authorized identities (=`Signatures` over `Public Keys`).

The primary identifier of an election is an authorized `Public Key`, that is unique for each election. The payload that is signed by the authority is the `Public Key` of the authorized voter concatenated with the election `Public Key` (which acts as salt). 

_Each authorized user may only vote once per election._

An eligible user may submit a vote for an election by generating a zero-knowledge proof where the public inputs are:

 - The `Public Key` associated with the election
 - The signed and salted user's public key
 - The user's vote selection (a string)

The private inputs to the zero-knowledge proof are:

 - A signature of the user's vote selection

## How It Works Exactly

We utilize a Risc0 guest program (circuit) to prove that a user possesses a `Private Key` with a corresponding `Public Key` that has been signed by an authority.
The user must sign the vote selection and the circuit will verify that "session `Signature`", as well as the government issued identity (which is also a `Signature`).

Therefore the workload that's handled inside the ZKVM is the verification of 2 `Signatures` for each proof of identity. The only information that is revealed is that a user possess a `Private Key` that corresponds to a `Public Key` that has been signed by the authority. The cryptographic identity of the user is not revealed to the public.

## The Risc0 circuit

The heart of this cryptographic protocol is the Risc0 circuit that takes the autorized `Public Key` as a secret input and the government issued identity (`Signature`) as a public input.

```rust
    let circuit_inputs: CircuitInputs = env::read();
    let choice: String = circuit_inputs.choice;
    let user_public_key: VerifyingKey = VerifyingKey::from_encoded_point(
        ...
    )
    .unwrap();
    let government_public_key: VerifyingKey = VerifyingKey::from_encoded_point(
        ...
    )
    .unwrap();

    user_public_key
        .verify(
            &circuit_inputs.government_public_key,
            &circuit_inputs.session_signature,
        )
        .expect("Failed to verify session signature");

    ...

    government_public_key
        .verify(
            &payload,
            &circuit_inputs.public_identity,
        )
        .expect("Failed to verify public identity");

    let output: CircuitOutputs = CircuitOutputs {
        choice: choice,
        government_public_key: circuit_inputs.government_public_key,
        public_identity: circuit_inputs.public_identity,
    };
    env::commit(&output);
```

## The Client

The Client can be used to issue identities, generate keypairs and submit votes.

```bash
cargo run -p acropolis
```

```
Usage: acropolis <COMMAND>

Commands:
  vote               
  generate-key-pair  
  issue-identity     
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

The Client crate has a `groth16` feature that indicates whether to submit a proof to the api and ethereum, or just the api.
Risc0-groth16 currently only supports x86 architecture and therefore this feature may not be enabled when running unsupported architecture.

## Auditing the API
Our API serves all `Elections` and their `Votes`. An external entity can utilize the functionality exposed by our `audit-utils` crate to verify all ZKPs (=votes) independently.

## Simple CLI Voting
First run the API and initialize Elections alongside with Accounts:
```rust
cargo run -p api
```

Open another terminal (or split tmux) and submit a vote:
```bash
cargo run -p acropolis --user-id-path ./election-1/user-1 --vote "dogs_and_cats"
```

Proving will take some time, once finished the API will serve the current state of the election.
To query elections:
```bash
http://127.0.0.1:8080/fetch_elections
```
To query all votes (+ ZK proofs) of an election that have been verified by the API:
```bash
http://127.0.0.1:8080/fetch_votes/<election_hex>
```

## Additional Contributions

We hopefully improved the developer experience by submitting a PR that packages kurtosis with Nix: https://github.com/kurtosis-tech/kurtosis/pull/2461
