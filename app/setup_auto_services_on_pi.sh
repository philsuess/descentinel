#!/bin/bash

sudo podman generate systemd --new --files --name descentinel --restart-policy=always --restart-sec=2
sudo mv *.service /lib/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable pod-descentinel.service

sudo cp mailbox/monitor /bin
sudo echo '[Unit]
Description=Descentinel monitor service - always watching...
After=pod-descentinel.service

[Service]
Type=simple
ExecStart=/bin/monitor -i 2500
Restart=on-failure
RestartSec=20

[Install]
WantedBy=default.target
' > /lib/systemd/system/descentinel-monitor.service
sudo systemctl enable descentinel-monitor.service


