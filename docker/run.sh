#!/bin/bash

redis-server &

until nc -z localhost 6379; do
        echo "Redis not ready yet. Waiting..."
        sleep 1
done

echo
echo
echo "\033[0;35mStarting CAPTCHA service ...\033[0m"

export RUST_LOG=rust_captcha=info
export REDIS_HOST=localhost

/home/dev/rust-captcha &

until nc -z localhost 8080; do
        echo "CAPTCHA service not ready yet. Waiting..."
        sleep 1
done

./test.sh

if [ $? -nq 0 ]; then
    echo "\033[0;31mError.\033[0m"
    exit 1
else
    echo "\033[0;32mReady.\033[0m"
    wait
fi
