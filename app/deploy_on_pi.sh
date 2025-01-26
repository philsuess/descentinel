#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

cd presentation
trunk build --release --features production
cd ..

readonly USER_ON_PI=phil
readonly TARGET_HOST=${USER_ON_PI}@descentinel
readonly USER_HOME_PATH=/home/${USER_ON_PI}
readonly TARGET_PATH=${USER_HOME_PATH}/mailbox
readonly EXCHANGE_FOLDER=${TARGET_HOST}:${TARGET_PATH}
readonly APPS_FOLDER=${EXCHANGE_FOLDER}/release


ssh ${TARGET_HOST} mkdir -p ${USER_HOME_PATH}/mailbox
ssh ${TARGET_HOST} mkdir -p ${USER_HOME_PATH}/mailbox/release
ssh ${TARGET_HOST} mkdir -p ${USER_HOME_PATH}/mailbox/html
ssh ${TARGET_HOST} mkdir -p ${USER_HOME_PATH}/.config/containers
ssh ${TARGET_HOST} mkdir -p ${USER_HOME_PATH}/.config/containers/systemd

scp -r ./presentation/dist/* ${EXCHANGE_FOLDER}/html

scp ./target/aarch64-unknown-linux-gnu/release/broadcast ${APPS_FOLDER}
scp ./services/broadcast.pi ${EXCHANGE_FOLDER}/broadcast

scp ./target/aarch64-unknown-linux-gnu/release/monitor ${APPS_FOLDER}

scp ./target/aarch64-unknown-linux-gnu/release/detect_card ${APPS_FOLDER}
scp ./services/detect_card.pi ${EXCHANGE_FOLDER}/detect_card

scp ./services/quadlets/* ${TARGET_HOST}:${USER_HOME_PATH}/.config/containers/systemd
scp ./services/systemd/descentinel-monitor.service ${TARGET_HOST}:${USER_HOME_PATH}/mailbox

scp ./run_on_pi.sh ${TARGET_HOST}:${USER_HOME_PATH}
scp ./setup_frontend.sh ${TARGET_HOST}:${USER_HOME_PATH}
