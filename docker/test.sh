#!/bin/bash

set -e

export PATH=../target/redis-3.2.9/src/:$PATH

ADDR="http://localhost:8080"

function ok {
    printf "%-40s [\033[0;32mok\033[0m]\n" "$@"
}

function err {
    printf "%-40s [\033[0;31merr\033[0m]\n" "$@"
}

# test cases
# 1) check that GET on endpoint / returns 404
# 2) check that GET to get a new CAPTCHA does not work (returns 404)
# 3) check that POST to get a new CAPTCHA does work
#    a) returns 200
#    b) returned id has length 36
#    c) uuid in redis is requal to id returned
#    d) number of tries_left in returned JSON is 3
#    e)

echo -e "\033[0;35mRunning some tests ...\033[0m"

# 1) check that endpoint "/" returns 404
n=$(curl -s -i -H 'X-CLIENT-ID: testing' $ADDR | grep "404 Not Found" | wc -l)
[ $n -eq 1 ] && ok "endpoint /" || err "endpoint /"

# 2) check that GET does not work to get a new CAPTCHA
n=$(curl -s -i -H 'X-CLIENT-ID: testing' $ADDR/new/easy/3/10 | grep "404 Not Found" | wc -l)
[ $n -eq 1 ] && ok "GET /new/..." || echo "GET /new/..."

# ---------------------------------------------------------------------------------------

# 3a) get a new CAPTCHA
t=$(mktemp)
n=$(curl -s -i -H 'X-CLIENT-ID: testing' -XPOST $ADDR/new/easy/3/30 > $t)
[ `cat $t | grep "200 OK" | wc -l` -eq 1 ] && ok "POST /new/..." || err "POST /new/..."

## 3b) check length of returned id (must be 36)
id=`cat $t | tail -n1 | jq -r .id`
n=`echo -n $id | wc -c`
[ $n -eq 36 ] && ok "check id" || err "check id"

## 3c) check uuid and number of tries left in redis
json=`redis-cli --raw get X1:$id`
uuid=`echo -n $json | jq -r .uuid`
[ "$uuid" == "$id" ] && ok "id equal" || echo "id equal"
## 3d)
tries=`echo -n $json | jq -r .tries_left`
[ $tries -eq 3 ] && ok "tries" || err "tries"

## 3e) 1st attempt
t=$(mktemp)
curl -s -i -H 'X-CLIENT-ID: testing' -XPOST $ADDR/solution/$uuid/1111111111 > $t
result=`cat $t | tail -n1 | jq -r .result`
reason=`cat $t | tail -n1 | jq -r .reject_reason`
[ "$result" == "rejected" ] && ok "1st attempt rejected" || err "1st attempt rejected"
[ "$reason" == "incorrect solution" ] && ok "1st attempt reason (incorrect)" || err "1st attempt reason (incorrect)"

## 2nd attempt
curl -s -i -H 'X-CLIENT-ID: testing' -XPOST $ADDR/solution/$uuid/1111111111 > $t
result=`cat $t | tail -n1 | jq -r .result`
reason=`cat $t | tail -n1 | jq -r .reject_reason`
[ "$result" == "rejected" ] && ok "2nd attempt rejected" || err "2nd attempt rejected"
[ "$reason" == "incorrect solution" ] && ok "2nd attempt reason (incorrect)" || err "2nd attempt reason (incorrect)"

## 3rd attempt
curl -s -i -H 'X-CLIENT-ID: testing' -XPOST $ADDR/solution/$uuid/1111111111 > $t
result=`cat $t | tail -n1 | jq -r .result`
reason=`cat $t | tail -n1 | jq -r .reject_reason`
[ "$result" == "rejected" ] && ok "3rd attempt rejected" || err "3rd attempt rejected"
[ "$reason" == "incorrect solution" ] && ok "3rd attempt reason (incorrect)" || err "3rd attempt reason (incorrect)"

## 4th attempt
curl -s -i -H 'X-CLIENT-ID: testing' -XPOST $ADDR/solution/$uuid/1111111111 > $t
result=`cat $t | tail -n1 | jq -r .result`
reason=`cat $t | tail -n1 | jq -r .reject_reason`
[ "$result" == "rejected" ] && ok "4th attempt rejected" || err "4th attempt rejected"
[ "$reason" == "too many trials" ] && ok "4th attempt reason (too many trials)" || err "4th attempt reason (too many trials)"

# ---------------------------------------------------------------------------------------

# get a new CAPTCHA
t=$(mktemp)
curl -s -i -H 'X-CLIENT-ID: testing' -XPOST $ADDR/new/easy/3/30 > $t
[ `cat $t | grep "200 OK" | wc -l` -eq 1 ] && ok "new captcha status code" || err "new captcha status code"
uuid=`cat $t | tail -n1 | jq -r .id`
json=`redis-cli --raw get X1:$uuid`
solution=`echo -n $json | jq -r .solution`

## check that solution is accepted
t=$(mktemp)
curl -s -i -H 'X-CLIENT-ID: testing' -XPOST $ADDR/solution/$uuid/$solution > $t
result=`cat $t | tail -n1 | jq -r .result`
[ "$result" == "accepted" ] && ok "correct solution accepted" || err "correct solution accepted"

## check that solution is rejected
curl -s -i -H "X-CLIENT-ID: testing" -XPOST $ADDR/solution/$uuid/$solution > $t
n=`cat $t | grep "404 Not Found" | wc -l`
[ $n -eq 1 ] && ok "correct solution valid only once" || err "correct solution valid only once"

