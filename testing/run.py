#!/usr/bin/env python3
import base64
import json
import re
import urllib
import urllib.request
import random
from io import BytesIO
from multiprocessing import Process
from PIL import Image

# In the root of this repository do:
# make redis (if you haven't a Redis instance already running)
# cargo run --release

# In this directory do:
# ./run.py

N_QUERIES = 1000
N_PROCESSES = 4


def get_captcha():
    req = urllib.request.Request('http://localhost:8000/new/easy/3/100', data=b"")
    req.add_header("X-Client-ID", "myclient")
    rsp = urllib.request.urlopen(req)
    data = json.loads(rsp.read())
    assert data["error_code"] == 0
    assert data["error_msg"] == "processed"
    result = data["result"]

    captcha_id = result["id"]
    assert re.match("^[0-9a-zA-Z-]{36,36}$", captcha_id)

    captcha_png = result["png"]
    assert re.match("^[0-9a-zA-Z+/=]{1000,}$", captcha_png)
    png = BytesIO(base64.b64decode(captcha_png))
    img = Image.open(png)
    img.verify()  # verify that this is a valid image
    assert img.format == "PNG"

    captcha_solution = result["solution"]
    assert re.match("^[0-9a-zA-Z]{3,10}$", captcha_solution)

    return captcha_id, captcha_solution


def check_solution(captcha_id, solution):
    req = urllib.request.Request(f"http://localhost:8000/solution/{captcha_id}/{solution}", data=b"")
    req.add_header("X-Client-ID", "myclient")
    rsp = urllib.request.urlopen(req)
    return json.loads(rsp.read())


def check_fail():
    captcha_id, captcha_solution = get_captcha()
    for i in [2, 1, 0]:
        j = check_solution(captcha_id, "xx")
        assert j["error_code"] == 0
        assert j["error_msg"] == "processed"
        assert j["result"]["solution"] == "incorrect"
        assert j["result"]["trials_left"] == i

    j = check_solution(captcha_id, "xx")
    assert j["error_code"] == 0
    assert j["error_msg"] == "processed"
    assert j["result"]["solution"] == "too many trials"
    assert j["result"]["trials_left"] == 0


def check_ok():
    captcha_id, captcha_solution = get_captcha()
    j = check_solution(captcha_id, captcha_solution)
    assert j["error_code"] == 0
    assert j["error_msg"] == "processed"
    assert j["result"]["solution"] == "accepted"
    assert j["result"]["trials_left"] == 0

    j = check_solution(captcha_id, captcha_solution)
    assert j["error_code"] == 0
    assert j["error_msg"] == "processed"
    assert j["result"]["solution"] == "not found"
    assert j["result"]["trials_left"] == 0


def perform_queries():
    for i in range(N_QUERIES):
        if random.randint(0, 1) == 0:
            check_ok()
        else:
            check_fail()


def main():
    print("Running tests...")
    procs = [Process(target=perform_queries) for _ in range(N_PROCESSES)]
    for p in procs:
        p.start()
    for p in procs:
        p.join()
    print("done")


if __name__ == "__main__":
    main()
