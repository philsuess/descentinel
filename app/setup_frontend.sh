apt install nginx
cd view
podman build --rm -t frontend_builder .
podman run --rm -v ./:/app frontend_builder
cd ..
rm -R /var/www/html/*
cp -r view/dist/* /var/www/html/