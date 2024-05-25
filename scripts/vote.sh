#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

ARGS=("$@")

USER_ID_PATH=$(realpath ${ARGS[0]})
VOTE=${ARGS[1]}

touch ${ARGS[2]}
RECEIPT_FILE=$(realpath ${ARGS[2]})

touch ${ARGS[3]}
VOTE_DATA_FILE=$(realpath ${ARGS[3]})

echo "Voting for $VOTE"
echo "--------------------"
start=`date +%s`
cargo run -p acropolis --features groth16 -- vote --user-id-path $USER_ID_PATH --vote $VOTE --receipt-out-path $RECEIPT_FILE --groth16-receipt-out-path $VOTE_DATA_FILE
end=`date +%s`
echo "--------------------"
echo "...took $((end - start)) seconds"

ELECTION_ID=$(cargo run -p acropolis -- extract-election-id --receipt-path $RECEIPT_FILE)

#echo "Outputting GROTH16 proof to ${VOTE_DATA_FILE}"
#echo "--------------------"
#start=`date +%s`
#cargo run -p acropolis --features groth16 -- groth16-proof --receipt-path $RECEIPT_FILE --out-path  $VOTE_DATA_FILE
#end=`date +%s`
#echo "--------------------"
#echo "...took $((end - start)) seconds"
# echo -n "0x03a896ff8dc100a10d21ed32cf33236cdd7e7fe13553ac0127c5eec31a3c9da980" > $VOTE_DATA_FILE

# GROTH_16_FILE="/tmp/groth16proof.txt"
# echo "0x" > $GROTH_16_FILE

# Upload groth16 proof to Ethereum using hardhat
echo "Uploading vote to Ethereum for election $ELECTION_ID"
(cd $SCRIPT_DIR/../ethereum;
 ELECTION_ID=$ELECTION_ID VOTE_DATA_FILE=$VOTE_DATA_FILE npm run vote)
