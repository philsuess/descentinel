# Prepare the Pi

1. Install the following on the pi (use a slim, headless distro):
   `sudo apt update; sudo apt upgrade -y; sudo apt install -y git podman open-infrastructure-container-tools`

# Deploy on Raspberry Pi

Run this on build computer (use `bash` instead of `sh` on ubuntu)

1. `cargo install cross`
1. `cross build --release --target=aarch64-unknown-linux-gnu`
1. `sh deploy_on_pi.sh`
1. ssh into pi and run `bash run_on_pi.sh`

## Manual testing

Run this on the pi

1. `sh run_on_pi.sh`
1. Visit `http://descentinel/` or `http://IP_ADDRESS_OF_PI_IN_LOCAL_NETWORK` in browser connected to same wifi as the pi

# Deploy locally

1. `cargo build --release`
1. `bash build_services.sh`
1. `bash create_pod.sh`
1. (optionally) `bash setup_frontend.sh`
1. `./target/release/monitor`
