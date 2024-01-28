# Deploy on Raspberry Pi

Run this on build computer (use `bash` instead of `sh` on ubuntu)

1. `sh build.sh`
1. `sh deploy_on_pi.sh`

Install the following on the pi (use a slim, headless distro):
`sudo apt update; sudo apt upgrade; sudo apt install git podman`

Run this on the pi

1. `sh run_on_pi.sh`
1. Visit `http://raspberrypi/` or `http://IP_ADDRESS_OF_PI_IN_LOCAL_NETWORK` in browser connected to same wifi as the pi

# Deploy locally

1. `sh build.sh`
1. `sh create_pod.sh`
1. (optionally) `sh setup_frontend.sh`
1. `./monitor_app`
