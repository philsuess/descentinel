## Requirements:

- opencv (`zypper install opencv-devel` on opensuse)

## Build for Raspberry Pi
### 3 (64-bit)
 To produce a binary for the raspberry pi (aarch64):
  1. `cargo install cross`
  2. `cross build --release --target=aarch64-unknown-linux-gnu` 

## Container:

### rabbitmq service

- `cargo build --release` to build the executable
- `podman build --rm --target identify_game_scene_service -t identify_game_scene_service .` to build the service
- `podman run --rm -p 5672:5672 -p 15672:15672 docker.io/rabbitmq:3.11-management` to start rabbitmq
- `podman run --rm -e RABBITMQ_AMQP_URL=0.0.0.0 --network host identify_game_scene_service` to run a single instance of the service

to test:
- `podman build --rm --target identify_game_scene_service_test -t identify_game_scene_service_test .`
- `podman run --rm -e RABBITMQ_AMQP_URL=0.0.0.0 --network host identify_game_scene_service_test` to test

### pod
- `podman pod create -p 15672:15672 -p 5672:5672 -p 3030:3030 --name descentinel`
- `podman run -d --pod descentinel --name rabbitmq docker.io/rabbitmq:3.11-management`
- `podman run -d --pod descentinel --name identify_game_scene_service -e RABBITMQ_AMQP_URL=0.0.0.0 identify_game_scene_service`

to test:
- `podman run --pod descentinel --name test_identify_game_scene -e RABBITMQ_AMQP_URL=0.0.0.0 identify_game_scene_service_test`