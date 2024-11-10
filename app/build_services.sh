#!/bin/bash

podman build --target broadcast_service --rm -v ${PWD}:/app -t broadcast_service -f services/broadcast .
podman build --target detect_card_service --rm -v ${PWD}:/app -t detect_card_service -f services/detect_card .