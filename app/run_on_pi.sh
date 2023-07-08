#!/bin/bash

cd mailbox
podman build --target broadcast_service --rm -t broadcast_service -f Containerfile.broadcast.service .
podman build --target detect_card_service --rm -t detect_card_service -f Containerfile.detect_card.service .
cd ..

sudo rm -r --force  descentinel/
git clone https://github.com/philsuess/descentinel
cd descentinel/app
sudo sh setup_frontend.sh

sh ./create_pod.sh
cd ../../

cd mailbox
for i in 1 2 3 4 5; do ./monitor && break || sleep 15; done
