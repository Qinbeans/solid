#!/bin/bash

# argument 1: run or build

if [ "$1" == "run" ]; then
    echo "Running Solid"
    cargo run -- -C link-args=-Wl,-zstack-size=2000000000
elif [ "$1" == "build" ]; then
    echo "Building Solid"
    cargo build -- -C link-args=-Wl,-zstack-size=2000000000
else
    echo "Invalid argument"
fi