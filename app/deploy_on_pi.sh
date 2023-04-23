#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@raspberrypi
readonly TARGET_PATH=/home/pi/mailbox
readonly SOURCE_PATH=./target/aarch64-unknown-linux-gnu/release/hello-world

cd broadcast
scp ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
cd ..

cd monitor
scp ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
cd ..
