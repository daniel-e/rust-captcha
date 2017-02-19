#!/bin/bash

set -e

# start redis
echo -e '\033[1;36m'"Starting redis ..."'\033[0m'
redis-server &

export PATH=$PATH:/opt/rust

# compile sources
echo -e '\033[1;36m'"Compiling sources ..."'\033[0m'
cd /tmp/
git clone https://github.com/daniel-e/rust-captcha.git
cd rust-captcha
cargo build --release
cargo test

# start service
echo -e '\033[1;36m'"Starting CAPTCHA service ..."'\033[0m'
./target/release/rust-captcha
