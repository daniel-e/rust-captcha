#!/bin/bash

set -e

export PATH=../target/redis-3.2.9/src/:$PATH

ADDR="http://localhost:8080"

# check that endpoint "/" returns 404
n=$(curl -s -i $ADDR | grep "404 Not Found" | wc -l)
[ $n -eq 1 ] && echo "ok" || echo "failed"

# check that GET does not work to get a new CAPTCHA
n=$(curl -s -i $ADDR/new/easy/3/10 | grep "404 Not Found" | wc -l)
[ $n -eq 1 ] && echo "ok" || echo "failed"

# ---------------------------------------------------------------------------------------

# get a new CAPTCHA
t=$(mktemp)
n=$(curl -s -i -XPOST $ADDR/new/easy/3/30 > $t)
[ `cat $t | grep "200 OK" | wc -l` -eq 1 ] && echo "status code ok" || echo "failed"

## check length of returned id (must be 36)
id=`cat $t | tail -n1 | jq -r .id`
n=`echo -n $id | wc -c`
[ $n -eq 36 ] && echo "id check ok" || echo "failed"

## check uuid and number of tries left in redis
json=`redis-cli --raw get X1:$id`
echo $json
# TODO DENIED Redis is running in protected mode because protected mode is enabled,  ....
uuid=`echo -n $json | jq -r .uuid`
[ "$uuid" == "$id" ] && echo "id equal ok" || echo "failed"
tries=`echo -n $json | jq -r .tries_left`
[ $tries -eq 3 ] && echo "tries ok" || echo "failed"

## 1st attempt
t=$(mktemp)
n=$(curl -s -i -XPOST $ADDR/solution/$uuid/1111111111 > $t)
result=`cat $t | tail -n1 | jq -r .result`
reason=`cat $t | tail -n1 | jq -r .reject_reason`
[ "$result" == "rejected" ] && echo "rejected ok" || echo "failed"
[ "$reason" == "incorrect solution" ] && echo "reason ok" || echo "failed"

## 2nd attempt
n=$(curl -s -i -XPOST $ADDR/solution/$uuid/1111111111 > $t)
result=`cat $t | tail -n1 | jq -r .result`
reason=`cat $t | tail -n1 | jq -r .reject_reason`
[ "$result" == "rejected" ] && echo "rejected ok" || echo "failed"
[ "$reason" == "incorrect solution" ] && echo "reason ok" || echo "failed"

## 3rd attempt
n=$(curl -s -i -XPOST $ADDR/solution/$uuid/1111111111 > $t)
result=`cat $t | tail -n1 | jq -r .result`
reason=`cat $t | tail -n1 | jq -r .reject_reason`
[ "$result" == "rejected" ] && echo "rejected ok" || echo "failed"
[ "$reason" == "incorrect solution" ] && echo "reason ok" || echo "failed"

## 4th attempt
n=$(curl -s -i -XPOST $ADDR/solution/$uuid/1111111111 > $t)
result=`cat $t | tail -n1 | jq -r .result`
reason=`cat $t | tail -n1 | jq -r .reject_reason`
[ "$result" == "rejected" ] && echo "rejected ok" || echo "failed"
[ "$reason" == "too many trials" ] && echo "reason ok" || echo "failed: $reason"

# ---------------------------------------------------------------------------------------

# get a new CAPTCHA
t=$(mktemp)
n=$(curl -s -i -XPOST $ADDR/new/easy/3/30 > $t)
[ `cat $t | grep "200 OK" | wc -l` -eq 1 ] && echo "status code ok" || echo "failed"
uuid=`cat $t | tail -n1 | jq -r .id`
json=`redis-cli --raw get X1:$uuid`
solution=`echo -n $json | jq -r .solution`

t=$(mktemp)
n=$(curl -s -i -XPOST $ADDR/solution/$uuid/$solution > $t)
result=`cat $t | tail -n1 | jq -r .result`
[ "$result" == "accepted" ] && echo "accepted ok" || echo "failed: $result, $uuid, $solution"

# TODO check that it does not work again
