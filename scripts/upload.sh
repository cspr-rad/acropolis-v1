#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

ARGS=("$@")

touch ${ARGS[0]}
RECEIPT_FILE=$(realpath ${ARGS[0]})

touch ${ARGS[1]}
VOTE_DATA_FILE=$(realpath ${ARGS[1]})

ELECTION_ID=$(cargo run -p acropolis -- extract-election-id --receipt-path $RECEIPT_FILE)

# Upload groth16 proof to Ethereum using hardhat
echo "Uploading vote to Ethereum for election $ELECTION_ID"
(cd $SCRIPT_DIR/../ethereum;
 ELECTION_ID=$ELECTION_ID VOTE_DATA_FILE=$VOTE_DATA_FILE npm run vote)
