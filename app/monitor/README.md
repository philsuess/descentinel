This service will

1. search for camera devices and start streaming using the first one found
2. it sends images to a specified queue to a specified ampq url in a specified time interval.

## Requirements:

rust

`cargo build -r` to build

## Service:

- run `monitor --help` for a list of options.

Note that running `monitor` in a container will not work as-is. The hardware search for the camera will fail. Rather, `monitor` should run natively on the pi. Here's how to build it:

  1. `podman build --rm -t monitor_build -f Containerfile .`
  1. `podman run -v ./:/monitor monitor_build` 
