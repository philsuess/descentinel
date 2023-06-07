#!/bin/bash

cargo build --release
podman build --rm --target detect_card_service --rm -t detect_card_service .

cross build --release --target=aarch64-unknown-linux-gnu
