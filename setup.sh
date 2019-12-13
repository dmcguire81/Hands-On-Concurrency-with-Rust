#!/usr/bin/env bash

apt-get update
apt-get install -y lldb gdb valgrind gcc-arm-linux-gnueabihf

rustup target add armv7-unknown-linux-gnueabihf

cat << CONFIG > ${CARGO_HOME}/config
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
CONFIG
