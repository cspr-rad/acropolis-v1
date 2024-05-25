#!/usr/bin/env bash

set -euo pipefail

kurtosis run github.com/kurtosis-tech/ethereum-package --enclave hardhat-enclave '{"additional_services": []}'