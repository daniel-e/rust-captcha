#!/bin/bash

set -e
cd /tmp/
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH=$PATH:~/.cargo/bin/
git clone https://github.com/daniel-e/rust-captcha.git
cd rust-captcha
cargo build
cargo test
echo -e '\033[1;32m'"TEST OK"'\033[0m'
exit 0
