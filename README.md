# Acropolis - a ZKVM enabled voting mechanism

![Demo with TUI and Ethereum Audit](https://github.com/cspr-rad/acropolis/blob/main/resources/demo-video.gif)

## Authorized Anonymous Voting

Acropolis enables anonymous voting through `Signatures` over `Public Keys`. The cryptographic identity of each user is treated as a secret input and never revealed to the public. Only the `Signature` issued by an authority for a "KYC-ed" user, alongside with the corresponding `Public Key` (one `Public Key` per election that is owned by the authority) are published for each election. An election is defined as a cycle with a fixed set of options to choose from and authorized identities (=`Signatures` over `Public Keys`).

The primary identifier of an election is an authorized `Public Key`, that is unique for each election. The payload that is signed by the authority is the `Public Key` of the authorized voter concatenated with the election `Public Key` (which acts as salt). 

_Each authorized user may only vote once per election._

An eligible user may submit a vote for an election by generating a zero-knowledge proof where the public inputs are:

 - The authority's `Public Key` associated with the election
 - The salted signature of the user's public key produced by the authority
 - The user's vote selection (a string)

The private inputs to the zero-knowledge proof are:

 - The user's `Public Key`
 - A signature of the user's vote selection

## How It Works Exactly

We utilize a risc0 guest program (circuit) to construct the proof.

The risc0 guest program performs the following:

 1. Validates, using the _authority's_ `Public Key`, the signature of the user's public key concatenated with the authority's `Public Key`
 2. Validates, using the _user's_ `Public Key`, the signature of the user's vote selection

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
The client exposes an `audit` endpoint that can be used to verify a set of votes. We scrape the votes the Smart Contract and dump them to a file that is fully auditable. The audit will evaluate all votes, verify all Zero Knowledge Proofs, flag malicious voters and output legitimate election results.

## Simple CLI Voting

First run the API and initialize Elections alongside with Accounts:

```bash
cargo run -p api
```

Open another terminal (or split tmux) and submit a vote:

```bash
cargo run -p acropolis -- --user-id-path ./election-1/user-1 --vote "dogs_and_cats"
```

Proving will take some time, once finished the API will serve the current state of the election.
To query elections:

```
http://127.0.0.1:8080/fetch_elections
```

To query all votes (+ ZK proofs) of an election that have been verified by the API:

```
http://127.0.0.1:8080/fetch_votes/<election_hex>
```

## Get an overview (TUI)

Again, make sure the API is running first:

```bash
cargo run -p api
```

Then run (in another shell):
```bash
cargo run -p tui
```

You'll get a window with a title that will show any errors (there won't be any if the API is running), a list of ongoing elections you can scroll through with arrow keys, and a bar graph showing the current results on teh right.

## Additional Contributions

We hopefully improved the developer experience by submitting a PR that packages kurtosis with Nix: https://github.com/kurtosis-tech/kurtosis/pull/2461
