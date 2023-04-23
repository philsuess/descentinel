#!/bin/bash

apt install nginx
cd view
podman build --rm -t frontend_builder .
podman run --rm -v ./:/app frontend_builder
podman image rm docker.io/library/node
podman image rm localhost/frontend_builder
cd ..
touch /var/www/html/dummy
rm -R /var/www/html/*
cp -r view/dist/* /var/www/html/
