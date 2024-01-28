#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=phil@raspberrypi.local
readonly TARGET_PATH=/home/phil/mailbox

ssh ${TARGET_HOST} mkdir -p ${TARGET_PATH}
ssh ${TARGET_HOST} mkdir -p ${TARGET_PATH}/release

cd broadcast
scp ./target/aarch64-unknown-linux-gnu/release/broadcast ${TARGET_HOST}:${TARGET_PATH}/release/
scp ./Containerfile.pi ${TARGET_HOST}:${TARGET_PATH}/Containerfile.broadcast.service
cd ..

cd monitor
scp ./target/aarch64-unknown-linux-gnu/release/monitor ${TARGET_HOST}:${TARGET_PATH}
cd ..

cd identify_game_scene
scp ./target/arm-unknown-linux-gnueabihf/release/identify_game_scene ${TARGET_HOST}:${TARGET_PATH}/release
scp ./OL_template.jpg ${TARGET_HOST}:${TARGET_PATH}
scp ./Containerfile.pi ${TARGET_HOST}:${TARGET_PATH}/Containerfile.identify_game_scene.service
cd ..

cd detect_card
scp ./target/aarch64-unknown-linux-gnu/release/detect_card ${TARGET_HOST}:${TARGET_PATH}/release
scp ./keywords_cards.json ${TARGET_HOST}:${TARGET_PATH}
scp ./Containerfile.pi ${TARGET_HOST}:${TARGET_PATH}/Containerfile.detect_card.service
cd ..

scp ./run_on_pi.sh ${TARGET_HOST}:${TARGET_PATH}/..
