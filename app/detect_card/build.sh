#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

podman build --target rabbit_service --rm -t detect_card_service .