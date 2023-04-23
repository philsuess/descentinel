cd broadcast
sh ./build.sh
podman image rm localhost/broadcast_builder
cd ..

cd monitor
sh ./build.sh
podman image rm localhost/monitor_builder
cd ..
cp monitor/target/release/monitor ./monitor_app

podman image rm docker.io/library/rust

cd detect_card
sh ./build.sh
podman image rm docker.io/library/python
cd ..
