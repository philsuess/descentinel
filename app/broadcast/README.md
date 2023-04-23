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

- `cargo build --release` to build the binary
- `podman build --target broadcast_service --rm -t broadcast_service .` to build the service
- `podman run -d -p 3030:3030 --name broadcast broadcast_service` to run it

# Build for Raspberry Pi
## 3 (64-bit)
 To produce a binary for the raspberry pi (aarch64):
  1. `cargo install cross`
  2. `cross build --release --target=aarch64-unknown-linux-gnu` 




