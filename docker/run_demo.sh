#!/bin/bash

set -e

# start redis
echo -e '\033[1;36m'"Starting redis ..."'\033[0m'
redis-server >/dev/null &

sleep 1

# start service
echo -e '\033[1;36m'"Starting CAPTCHA service ..."'\033[0m'
/tmp/rust-captcha/target/release/rust-captcha -c /tmp/config.json
