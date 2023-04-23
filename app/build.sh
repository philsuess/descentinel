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
