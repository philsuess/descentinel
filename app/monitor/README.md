This service will

1. search for camera devices and start streaming using the first one found
2. it sends images to a specified queue to a specified ampq url in a specified time interval.

## Requirements:

rust

`cargo build -r` to build

## Service:

- run `monitor --help` for a list of options.

Note that running `monitor` in a container will not work as-is. The hardware search for the camera will fail. Rather, `monitor` should run natively on the pi. Here's how to build it:

- To produce a binary for the raspberry pi (aarch64):
  1. `podman build --target monitor_build --rm -t monitor_build -f Containerfile.aarch64 .` to build the cross compilation container
  1. `podman run -v <path_to_app>\monitor:/monitor monitor_build` to build the aarch64 version of the app
  1. Resulting binary will be in `target/aarch64-unknown-linux-gnu/release/monitor`

Caused by:
process didn't exit successfully: `/monitor/target/debug/build/v4l2-sys-mit-2c41a0547a3a2aa6/build-script-build` (exit status: 101)
--- stderr
/usr/include/linux/videodev2.h:60:10: fatal error: 'sys/time.h' file not found
/usr/include/linux/videodev2.h:60:10: fatal error: 'sys/time.h' file not found, err: true
thread 'main' panicked at 'Failed to generate bindings: ()', /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/v4l2-sys-mit-0.2.0/build.rs:10:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
