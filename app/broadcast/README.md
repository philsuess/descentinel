This is a warp server that broadcasts all backend content, specifically the last item in the rabbitmq queues is provided.

# Routes

- `health`: am I dead?
- `descentinel/`
  - `log`: last log entry
  - `game_room_image`: last image from game room
  - `detected_ol_card`: top secret last detected OL card

# Dev helper

`broadcast --help`

`RUST_LOG=info ...`

# Service (container me)

- `podman build --target broadcast_service --rm -t broadcast_service .` to build the service

- To produce a binary for the raspberry pi (aarch64):
  1. `podman build --rm -t broadcast_build_aarch64 -f Containerfile.aarch64 .` to build the cross compilation container
  1. `podman run -v <path_to_app>\broadcast:/broadcast broadcast_build_aarch64` to build the aarch64 version of the app
  1. Resulting binary will be in `target/aarch64-unknown-linux-gnu/release/broadcast`
