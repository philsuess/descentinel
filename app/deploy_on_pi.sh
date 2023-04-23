#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@raspberrypi
readonly TARGET_PATH=/home/pi/mailbox

cd broadcast
scp ./target/aarch64-unknown-linux-gnu/release/broadcast ${TARGET_HOST}:${TARGET_PATH}/release/
scp ./Containerfile ${TARGET_HOST}:${TARGET_PATH}/Containerfile.broadcast.service
cd ..

cd monitor
scp ./target/aarch64-unknown-linux-gnu/release/monitor ${TARGET_HOST}:${TARGET_PATH}
cd ..
