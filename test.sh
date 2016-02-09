#!/bin/bash

URL=localhost:8080
URL_CREATE=$URL/session
REDISCLI=target/redis-3.0.7/src/redis-cli
REDISPORT=6379

# Create a new CAPTCHA.
# Exptected response:
# - status code = 201
# - solved = false
# - tries = 0
# Returns the value of the Location header field.
function create_captcha {
  T=`mktemp`
  curl -s -i -X POST $URL_CREATE >$T

  # check status code
  statuscode=`cat $T | head -n1 | awk '{print $2}'`
  if [ "$statuscode" != "201" ]; then
    echo "$statuscode != 201" >/dev/stderr
    exit 1
  fi

  # check values in json body
  body=`cat $T | tail -n1 | egrep -e '^{.*}$'`
  x=`echo $body | jq -r .solved`
  if [ "$x" != "false" ]; then
    echo "$x != false" >/dev/stderr
    exit 1
  fi
  x=`echo $body | jq -r .tries`
  if [ "$x" != "0" ]; then
    echo "$x != 0" >/dev/stderr
    exit 1
  fi

  # return Location header
  cat $T | egrep -e '^Location: ' | awk '{print $2}' | tr -d '\r\n'
}

# Get a CAPTCHA.
# Parameters
# - Path of CAPTCHA
# - Expected status code
# - Expected value for solved
# - Expected value for tries
function get_captcha {
  path=$1
  expected_statuscode=$2
  expected_solved=$3
  expected_tries=$4
  T=`mktemp`
  curl -s -i -X GET "$URL$path" > $T
  if [ $? -ne 0 ]; then
    echo "curl failed" >/dev/stderr
    echo "curl -s -i -X GET $URL$path" >/dev/stderr
    exit 1
  fi

  # check status code
  statuscode=`cat $T | head -n1 | awk '{print $2}'`
  if [ "$statuscode" != "$expected_statuscode" ]; then
    echo "$statuscode != $expected_statuscode" >/dev/stderr
    exit 1
  fi

  # check values in json body
  body=`cat $T | tail -n1 | egrep -e '^{.*}$'`
  x=`echo $body | jq -r .solved`
  if [ "$x" != "$expected_solved" ]; then
    echo "$x != $expected_solved" >/dev/stderr
    exit 1
  fi
  x=`echo $body | jq -r .tries`
  if [ "$x" != "$expected_tries" ]; then
    echo "$x != $expected_tries" >/dev/stderr
    exit 1
  fi
}

# Parameters
# - Path of CAPTCHA
# - Expected status code
# - Solution
function check_captcha {
  p=$1
  expected_statuscode=$2
  solution=$3
  expected_info=$4
  expected_tries=$5

  json='{"solution": "'$solution'"}'
  T=`mktemp`
  curl -s -i -d "$json" -X POST "$URL$p" > $T
  if [ $? -ne 0 ]; then
    echo "curl failed" >/dev/stderr
    exit 1
  fi

  # check status code
  statuscode=`cat $T | head -n1 | awk '{print $2}'`
  if [ "$statuscode" != "$expected_statuscode" ]; then
    echo "$statuscode != $expected_statuscode" >/dev/stderr
    echo "$URL$p" > /dev/stderr
    exit 1
  fi

  # check values in json body
  body=`cat $T | tail -n1 | egrep -e '^{.*}$'`
  x=`echo $body | jq -r .info`
  if [ "$x" != "$expected_info" ]; then
    echo "$x != $expected_info" >/dev/stderr
    exit 1
  fi
  x=`echo $body | jq -r .tries`
  if [ "$x" != "$expected_tries" ]; then
    echo "$x != $expected_tries" >/dev/stderr
    exit 1
  fi
}

# -----------------------------------------------------------------------------

# TODO
# check status code if Redis is not available 503
echo "test 1"
path=`create_captcha`

echo "test 2"
get_captcha $path 200 "false" 0

# check validation error
echo "test 3"
get_captcha /session/abc 400

# check captcha not found
echo "test 4"
get_captcha /session/12345678901234567890 404

# check invalid json
echo "test 5"
ret=`$REDISCLI set aaaaabbbbbcccccddddd 123`
if [ $ret != "OK" ]; then
  echo "redis-cli failed"
  exit 1
fi
get_captcha /session/aaaaabbbbbcccccddddd 500

#echo "test 6"
## check Redis not available
#fuser -k $REDISPORT/tcp >/dev/null 2>/dev/null
#sleep 1
#get_captcha $path 503

echo "test 7"
path=`create_captcha`
check_captcha $path 400 a
check_captcha $path 400 12345:
check_captcha /session/aaaaabbbbbcccccddddx/ 404 12345
check_captcha /session/aaaaabbbbbcccccdddd/ 400 12345

check_captcha $path 200 12345 "Incorrect." 1
check_captcha $path 200 12345 "Incorrect." 2
check_captcha $path 200 12345 "Incorrect." 3
check_captcha $path 200 12345 "Incorrect." 4
check_captcha $path 200 12345 "Too many tries." 4
check_captcha $path 200 12345 "Too many tries." 4

path=`create_captcha`
session=`echo $path | cut -d/ -f3`
sol=`$REDISCLI --raw get $session | jq -r .solution`
check_captcha $path 200 12345 "Incorrect." 1
check_captcha $path 200 12345 "Incorrect." 2
check_captcha $path 200 12345 "Incorrect." 3
check_captcha $path 200 $sol "Correct." 4
check_captcha $path 200 $sol "Too many tries." 4
check_captcha $path 200 $sol "Too many tries." 4
