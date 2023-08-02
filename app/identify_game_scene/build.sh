#!/bin/bash

cargo build --release
podman build --rm --target identify_game_scene_service --rm -t identify_game_scene_service .

cross build --release --target=aarch64-unknown-linux-gnu
