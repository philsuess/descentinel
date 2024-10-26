#!/bin/bash

podman build --target broadcast_service --rm -v ${PWD}:/app -t broadcast_service -f services/broadcast .