#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

cd mailbox
podman build --target broadcast_service --rm -t broadcast_service .
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
