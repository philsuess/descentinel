#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

cargo install cross
cargo install --release

cd broadcast
sh ./build.sh
cd ..

cd monitor
sh ./build.sh
cd ..
cp monitor/target/release/monitor ./monitor_app

cd identify_game_scene
sh ./build.sh
cd ..

cd detect_card
sh ./build.sh
cd ..
