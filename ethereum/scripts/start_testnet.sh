#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

kurtosis run github.com/kurtosis-tech/ethereum-package --enclave hardhat-enclave '{"additional_services": []}'

PORT=$(kurtosis enclave inspect hardhat-enclave | grep "rpc: 8545/tcp" | grep -oh "127.0.0.1:[0-9]*" | cut -d':' -f2)

echo $PORT > $SCRIPT_DIR/../PORT