#!/bin/bash

redis-server &
sleep 1

echo
echo
echo "Starting CAPTCHA service ..."

export RUST_LOG=rust_captcha=info
export REDIS_HOST=localhost

/home/dev/rust-captcha