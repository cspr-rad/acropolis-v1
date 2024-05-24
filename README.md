# Acropolis - a ZKVM enabled voting mechanism

## Anonymous voting abstract
Acropolis enables anonymous voting through `Signatures` over `Public Keys`. The cryptographic identity of each user is treated as a secret input and never revealed to the public. Only the `Signature` issued by the autorities for a "KYC-ed" user, alongside with the corresponding `Public Key` (one `Public Key` per election that is owned by the authority) are published for each election. An election is defined as a cycle with a fixed set of options to choose from and government issued identities (=`Signatures` over `Public Keys`).

From this point onward the term `authority` will be used interchangably with `government` for the sake of simplicity. It is however not important what centralized or decentralized entity issues the identities (`Signatures`). In the context of an election the authority could be the government.

An eligible user may submit a vote for an election by generating a proof where the public inputs are their government issued identity, alongside with a `Signature` over the government `Public Key` associated with that election.

## How it works exactly
We utilize a Risc0 guest program (circuit) to prove that a user possesses a `Private Key` with a corresponding `Public Key` that has been signed by an authority.
The user must sign the government issued `Public Key` (remember, one `Public Key` is issued per election) and the circuit will verify that "session `Signature`", as well as the government issued identity (which is also a `Signature`).

Therefore the workload that's handled inside the ZKVM is the verification of 2 `Signatures` for each proof of identity. The only information that is revealed is that a user possess a `Private Key` that corresponds to a `Public Key` that has been signed by the authority. The cryptographic identity of the user is not revealed to the public.

## Trust assumptions
This protocol is based on the trust assumption that the government / authority handles a Keypair as intended and does not leak it for at least until the election was concluded. Additionally, this protocol relies on ECDSA `Signatures` being irreversible / one-way functions.

