#!/bin/bash

cd mailbox
sudo podman build --target broadcast_service --rm -t broadcast_service -f Containerfile.broadcast.service .
sudo podman build --target identify_game_scene_service --rm -t identify_game_scene_service -f Containerfile.identify_game_scene.service .
sudo podman build --target detect_card_service --rm -t detect_card_service -f Containerfile.detect_card.service .
cd ..

sudo rm -r --force  descentinel/
git clone https://github.com/philsuess/descentinel
cd descentinel/app
sudo sh setup_frontend.sh

sh ./create_pod.sh

#cd ../../
#cd mailbox
#for i in 1 2 3 4 5; do sudo ./monitor && break || sleep 15; done


# is_pod_running = 0
# while is_pod_running == 0; do sleep(2) && pod_running=$(expr $(sudo podman pod ps | grep "descentinel" | grep -c "Running") == 1); done
#sudo ./monitor

sh ./setup_auto_services_on_pi.sh
sudo reboot 
