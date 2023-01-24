## Requirements:

- python
- poetry
- tesseract (https://github.com/tesseract-ocr/tesseract)

## Container:

### Tests
- `podman build --target=acceptance_tests -t detect_card_test --rm .`
- `podman run detect_card_test`
