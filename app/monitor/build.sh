#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

podman build --rm -t monitor_build -f Containerfile .
podman run -v ./:/monitor monitor_build

cross build --release --target=aarch64-unknown-linux-gnu