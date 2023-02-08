## Requirements:

- python
- tesseract (https://github.com/tesseract-ocr/tesseract)

## Container:

### rabbitmq service

- `podman build --target rabbit_service --rm -t detect_card_service .` to build the service
- `podman run --rm -p 5672:5672 -p 15672:15672 docker.io/rabbitmq:3.11-management` to start rabbitmq
- `podman run -e RABBITMQ_AMQP_URL=0.0.0.0 --network host detect_card_service` to run a single instance of the service
- `docker-compose up` to test the service

### pod
- `podman pod create -p 15672:15672 --name descentinel`
- `podman run -d --pod descentinel --name rabbitmq docker.io/rabbitmq:3.11-management`
- `podman run -d --pod descentinel --name detect_card -e RABBITMQ_AMQP_URL=0.0.0.0 detect_card_service`

to test:
- `podman run --pod descentinel --name test_detect_card -e RABBITMQ_AMQP_URL=0.0.0.0 detect_card_service_test`

### Tests

#### Acceptance tests

- `podman build --target=acceptance_tests -t detect_card_test --rm .` to build the test container
- `podman run detect_card_test` to run the tests

#### Services tests

- `podman build --target rabbit_service_test --rm -t detect_card_service_test .` to build the test service
- make sure rabbitmq and detect_card_service are running
- `podman run -e RABBITMQ_AMQP_URL=0.0.0.0 --network host detect_card_service_test` to run a single instance of the service
