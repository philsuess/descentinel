This is a warp server that broadcasts all backend content, specifically the last item in the rabbitmq queues is provided.

# Routes

Each route is served by server-sent events.

- `Q_GAMEQ_GAME_ROOM_FEED`: current image of camera capture
- `Q_SHORT_LOG`: logs
- `Q_DETECTED_OL_CARDS`: last detected OL card

# Dev helper

`broadcast --help`

`RUST_LOG=info ...`

# Service (container me)

from the `app` directory, run

`podman build --target broadcast_service --rm -v ${PWD}:/app -t broadcast_service -f services/broadcast .` to build the service

Then
- `podman run -d -p 3030:3030 --name broadcast broadcast_service` to run it
- `podman run -d --pod descentinel --name broadcast broadcast_service` to run it in the pod

# Build for Raspberry Pi

## 3/4 (64-bit)

To produce a binary for the raspberry pi (aarch64):

1. `cargo install cross`
2. `cross build --release --target=aarch64-unknown-linux-gnu`
