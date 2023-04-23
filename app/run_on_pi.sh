#!/bin/bash

cd mailbox
podman build --target broadcast_service --rm -t broadcast_service -f Containerfile.broadcast.service .
cd ..

git clone https://github.com/philsuess/descentinel
cd descentinel/app
sudo sh setup_frontend.sh

cd detect_card
sh build.sh
cd ..

sh ./create_pod.sh
cd ../../

cd mailbox
./monitor
