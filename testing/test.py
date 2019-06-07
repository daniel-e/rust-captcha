#!/usr/bin/env python3
import time
import urllib
import urllib.request
import sys
from multiprocessing import Process

n = 1000
k = int(sys.argv[1])

def send_req():
    req = urllib.request.Request('http://localhost:8080/new/easy/3/100', data=b"")
    req.add_header("X-Client-ID", "myclient")
    rsp = urllib.request.urlopen(req)
    _data = rsp.read()
    return rsp.getcode()


def f():
    t1 = time.time()
    for i in range(n):
        if send_req() != 200:
            print("error")
    t2 = time.time()
    print("process", n, t2 - t1)


def main():
    procs = [Process(target=f) for _ in range(k)]
    print("starting...")
    t1 = time.time()
    for p in procs:
        p.start()
    print("started")
    for p in procs:
        p.join()
        print("process done")
    t2 = time.time()
    print("done")
    print("{} threads, {} captchas in {} seconds = {} captchas/s".format(k, n * k, t2 - t1, n * k / (t2 - t1)))


if __name__ == "__main__":
    main()
