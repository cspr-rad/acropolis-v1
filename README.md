# Acropolis - a ZKVM enabled voting mechanism

## Authorized Anonymous Voting
Acropolis enables anonymous voting through `Signatures` over `Public Keys`. The cryptographic identity of each user is treated as a secret input and never revealed to the public. Only the `Signature` issued by the autorities for a "KYC-ed" user, alongside with the corresponding `Public Key` (one `Public Key` per election that is owned by the authority) are published for each election. An election is defined as a cycle with a fixed set of options to choose from and government issued identities (=`Signatures` over `Public Keys`).

From this point onward the term `authority` will be used interchangably with `government` for the sake of simplicity. It is however not important what centralized or decentralized entity issues the identities (`Signatures`). In the context of an election the authority could be the government.

The primary identifier of an election is the government issued `Public Key` that is unique for each election. The payload that is signed by the government is the `Public Key` of the authorized voter concatenated with the government `Public Key` (which acts as salt to prevent reverse,- and social engineering). 

`Each authorized user may only vote once per election and currently the weight of all votes is 1.`

An eligible user may submit a vote for an election by generating a proof where the public inputs are their government issued identity, alongside with a `Signature` over the government `Public Key` associated with that election.

## How it works exactly
We utilize a Risc0 guest program (circuit) to prove that a user possesses a `Private Key` with a corresponding `Public Key` that has been signed by an authority.
The user must sign the government issued `Public Key` (remember, one `Public Key` is issued per election) and the circuit will verify that "session `Signature`", as well as the government issued identity (which is also a `Signature`).

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