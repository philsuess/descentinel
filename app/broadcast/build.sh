#!/bin/bash

cargo build --release
podman build --target broadcast_service --rm -t broadcast_service .

cross build --release --target=aarch64-unknown-linux-gnu
