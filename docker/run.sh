#!/bin/bash

echo -e "\033[0;35mStarting Redis ...\033[0m"

sleep 1

until nc -z localhost 6379; do
        echo "Redis not ready yet. Waiting..."
        sleep 1
done

echo -e "\033[0;35mStarting CAPTCHA service ...\033[0m"

export RUST_LOG=rust_captcha=info
export REDIS_HOST=localhost

(/home/dev/rust-captcha 2>&1 | grep -v "testing") &
sleep 1

until nc -z localhost 8080; do
        echo "CAPTCHA service not ready yet. Waiting..."
        sleep 1
done

./test.sh

if [ $? -ne 0 ]; then
    echo -e "\033[0;31mError.\033[0m"
    exit 1
else
    echo -e "\033[0;32mReady.\033[0m"
    while true; do sleep 10; done
fi
