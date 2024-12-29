#!/bin/bash

sudo apt update; sudo apt upgrade -y; sudo apt install -y git podman 

cd mailbox
sudo podman build --target broadcast_service --rm -t broadcast -f broadcast .
sudo podman build --target detect_card_service --rm -t detect_card -f detect_card .

sudo podman run -d --network=host --name rabbitmq docker.io/rabbitmq:4-management
sudo podman generate systemd --new --files --name rabbitmq --restart-policy=always --start-timeout 600
sudo podman run -d --name detect_card --network=host  -e RUST_LOG=info detect_card
sudo podman generate systemd --new --files --name detect_card --restart-policy=always --requires container-rabbitmq.service --after container-rabbitmq.service --start-timeout 600
sed -i "/\[Service\]/a StartLimitBurst=10000" container-detect_card.service 
sudo podman run -d --network=host --name broadcast -e RUST_LOG=info broadcast
sudo podman generate systemd --new --files --name broadcast --restart-policy=always --requires container-rabbitmq.service --after container-rabbitmq.service --start-timeout 600
sed -i "/\[Service\]/a StartLimitBurst=10000" container-broadcast.service 

sudo sudo mv *.service /lib/systemd/system/
sudo cp /home/phil/mailbox/release/monitor /bin

sudo systemctl daemon-reload
sudo systemctl enable container-rabbitmq.service
sudo systemctl enable container-detect_card.service
sudo systemctl enable container-broadcast.service
sudo systemctl enable descentinel-monitor.service

cd ..

sudo rm -r --force  descentinel/
git clone https://github.com/philsuess/descentinel
cd descentinel/app
#sudo sh setup_frontend.sh

#sudo bash ./create_pod.sh

#cd ../../
#cd mailbox
#for i in 1 2 3 4 5; do sudo ./monitor && break || sleep 15; done


# is_pod_running = 0
# while is_pod_running == 0; do sleep(2) && pod_running=$(expr $(sudo podman pod ps | grep "descentinel" | grep -c "Running") == 1); done
#sudo ./monitor

sudo reboot 
