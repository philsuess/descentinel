#!/bin/bash

podman pod create -p 15672:15672 -p 5672:5672 -p 3030:3030 --name descentinel
podman run -d --pod descentinel --name rabbitmq docker.io/rabbitmq:3.11-management
podman run -d --pod descentinel --name detect_card -e RABBITMQ_AMQP_URL=0.0.0.0 detect_card_service
podman run -d --pod descentinel --name broadcast -e RUST_LOG=info broadcast_service