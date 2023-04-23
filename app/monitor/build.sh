podman build --rm -t monitor_builder .
podman run --rm -v ./:/monitor monitor_builder