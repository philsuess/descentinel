#!/bin/bash

podman pod rm -f descentinel

podman pod create -p 15672:15672 -p 5672:5672 -p 3030:3030 --name descentinel
podman run -d --pod descentinel --restart=always --name rabbitmq docker.io/rabbitmq:3.12-management
podman run -d --pod descentinel --restart=always --name identify_game_scene -e RUST_LOG=info -e RABBITMQ_AMQP_URL=0.0.0.0 identify_game_scene_service
podman run -d --pod descentinel --restart=always --name detect_card -e RUST_LOG=info -e RABBITMQ_AMQP_URL=0.0.0.0 detect_card_service
podman run -d --pod descentinel --restart=always --name broadcast -e RUST_LOG=info broadcast_service