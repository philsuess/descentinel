#!/bin/bash

podman build --rm -t monitor_build -f Containerfile .
podman run --rm -v ./:/monitor monitor_build

cross build --release --target=aarch64-unknown-linux-gnu