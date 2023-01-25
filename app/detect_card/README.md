## Requirements:

- python
- tesseract (https://github.com/tesseract-ocr/tesseract)

## Container:

### rabbitmq service
- `podman build --target rabbit_service --rm -t detect_card_service .` to build the service
- `podman run detect_card_service` to run a single instance of the service
- `docker-compose up` to test the service

### Tests
- `podman build --target=acceptance_tests -t detect_card_test --rm .` to build the test container
- `podman run detect_card_test` to run the tests

