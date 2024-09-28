## Requirements:

- leptonica (`zypper install leptonica-devel` on opensuse, `sudo apt install libleptonica-dev` on ubuntu, `sudo apt install leptonica-devel.x86_64`)
- tesseract (`zypper install tesseract-ocr-devel tesseract-ocr-fra` on opensuse `sudo apt install libtesseract-dev tesseract-ocr-fra` on ubuntu, `sudo dnf install tesseract-devel.x86_64 tesseract-langpack-fra.noarch` on fedora)

use these:

### opensuse
`sudo zypper install leptonica-devel tesseract-ocr-devel tesseract-ocr-fra`

### ubuntu
`sudo apt install libleptonica-dev ibtesseract-dev tesseract-ocr-fra`

### fedora
`sudo dnf install leptonica-devel.x86_64 tesseract-devel.x86_64 tesseract-langpack-fra.noarch`

## Build for Raspberry Pi

### 3 (64-bit)

To produce a binary for the raspberry pi (aarch64):

1. `cargo install cross`
2. `cross build --release --target=aarch64-unknown-linux-gnu`

## Container:

### rabbitmq service

- `podman build --rm --target detect_card_service -t detect_card_service .` to build the service
- `podman run --rm -p 5672:5672 -p 15672:15672 docker.io/rabbitmq:3.12-management` to start rabbitmq
- `podman run --rm -e RABBITMQ_AMQP_URL=0.0.0.0 --network host detect_card_service` to run a single instance of the service

to test:

- `podman build --rm --target detect_card_service_test -t detect_card_service_test .`
- `podman run --rm -e RABBITMQ_AMQP_URL=0.0.0.0 --network host detect_card_service_test` to test (expect the card "Dark Balm" to be detected)

### pod

- `podman pod create -p 15672:15672 -p 5672:5672 -p 3030:3030 --name descentinel`
- `podman run -d --pod descentinel --name rabbitmq docker.io/rabbitmq:3.12-management`
- `podman run -d --pod descentinel --name detect_card -e RABBITMQ_AMQP_URL=0.0.0.0 detect_card_service`

to test (expect the card "Dark Balm" to be detected):

- `podman run --pod descentinel --name test_detect_card -e RABBITMQ_AMQP_URL=0.0.0.0 detect_card_service_test`
