#!/usr/bin/env bash

# Chapter 1

apt-get update
apt-get install -y lldb gdb valgrind

# https://www.raspberrypi.org/products/raspberry-pi-3-model-b-plus/
apt-get install -y gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf
rustup component add rustfmt

cat << CONFIG > ${CARGO_HOME}/config
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
CONFIG

# Chapter 2

# https://github.com/rust-fuzz/afl.rs
cargo install afl

# https://github.com/bheisler/criterion.rs#quickstart
apt-get install -y gnuplot

apt-get install -y linux-perf
