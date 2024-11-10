## Build for Raspberry Pi

### 3 (64-bit)

To produce a binary for the raspberry pi (aarch64):

1. `cargo install cross`
2. `cross build --release --target=aarch64-unknown-linux-gnu`

## To test isolated:

### set up python test:

In `detect_card/service_test`, run 
    1. `python -m venv .venv`
    1. `. .venv/bin/activate`
    1. `pip install pika`
    
### to test:

1. `podman run --rm -p 5672:5672 -p 15672:15672 docker.io/rabbitmq:4-management` to start rabbitmq
1. run the detect card service using `RUST_LOG=info cargo run --release`
1. in `service_test` run `. .venv/bin/activate`
1. on the command line run `Q_GAME_ROOM_FEED=Q_GAME_ROOM_FEED RABBITMQ_AMQP_URL=0.0.0.0 python send_an_image_to_rabbitmq.py`

## Container:

### rabbitmq service

- from the app directory, run `podman build --rm -t detect_card_service -f services/detect_card .` to build the service
- `podman run --rm -p 5672:5672 -p 15672:15672 docker.io/rabbitmq:4-management` to start rabbitmq
- `podman run --rm -e RABBITMQ_AMQP_URL=0.0.0.0 --network host detect_card_service` to run a single instance of the service

to test:

- from the detect_card directory, run `podman build --rm --target detect_card_service_test -t detect_card_service_test -f service_test/Containerfile .`
- `podman run --rm -e RABBITMQ_AMQP_URL=0.0.0.0 --network host detect_card_service_test` to test (expect the output "got overlordcard/doom from a decoding")

### pod

- `podman pod create -p 15672:15672 -p 5672:5672 -p 3030:3030 --name descentinel`
- `podman run -d --pod descentinel --name rabbitmq docker.io/rabbitmq:4-management`
- `podman run -d --pod descentinel --name detect_card -e RABBITMQ_AMQP_URL=0.0.0.0 detect_card_service`

to test (expect the card "got overlordcard/doom from a decoding" to be detected):

- `podman run --pod descentinel --name test_detect_card -e RABBITMQ_AMQP_URL=0.0.0.0 detect_card_service_test`
