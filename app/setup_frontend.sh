#!/bin/bash

apt install -y nginx
touch /var/www/html/dummy
rm -R /var/www/html/*
cp -r mailbox/html/* /var/www/html/
