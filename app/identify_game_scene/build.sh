#!/bin/bash

cargo update
cargo build --release
podman build --rm --target identify_game_scene_service --rm -t identify_game_scene_service .

# check for latest Containerfile here: https://github.com/twistedfall/opencv-rust/blob/master/tools/docker/rpi-xcompile.Dockerfile
podman build --rm -t rpi-xcompile -f Containerfile.cross_builder .
podman build --rm -t identify_game_scene_rpi_builder -f Containerfile.build.for.pi .
podman run --rm -v ./:/identify_game_scene identify_game_scene_rpi_builder
