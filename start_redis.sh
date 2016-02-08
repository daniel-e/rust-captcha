#!/bin/bash

BIN=$1

function check_redis {
  r=`target/redis-3.0.7/src/redis-cli set x 0`
  if [ "$r" == "OK" ]; then
    echo "Redis is running."
    exit 0
  fi
}

check_redis
$BIN &

while true; do
  check_redis
  sleep 0.2
done
