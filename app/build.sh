#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

cargo install cross

cd broadcast
sh ./build.sh
cd ..

cd monitor
sh ./build.sh
cd ..
cp monitor/target/release/monitor ./monitor_app

cd detect_card
sh ./build.sh
cd ..
