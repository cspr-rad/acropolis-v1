#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

ARGS=("$@")

ELECTION_ID=${ARGS[0]}
VOTE_SCRAPE_FILE=${ARGS[1]}

echo "Scraping votes from Ethereum for election $ELECTION_ID"
(cd $SCRIPT_DIR/../ethereum;
 ELECTION_ID=$ELECTION_ID VOTE_SCRAPE_FILE=$VOTE_SCRAPE_FILE npm run scrape)

cat $VOTE_SCRAPE_FILE