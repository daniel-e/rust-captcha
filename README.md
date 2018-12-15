[![Build Status](https://travis-ci.org/daniel-e/rust-captcha.svg?branch=master)](https://travis-ci.org/daniel-e/rust-captcha)

# RESTful CAPTCHA Service

A RESTful CAPTCHA service written in Rust. The service generates CAPTCHAs
which can be embedded into web pages to protect them from being
accessed by bots. The difficulty of the CAPTCHAs, the expiration time and
the maximum number of attempts to solve a CAPTCHA can be easily
configured for each created CAPTCHA separately.

The CAPTCHAs look similar to the following ones:

![captcha](doc/captcha3.png) &nbsp; ![captcha](doc/captcha2.png) &nbsp; ![captcha](doc/captcha_mila_medium.png)

## Running

There are two ways how you can run the service.

### In Docker

```bash
cd docker
make build    # needs to be executed for the first time only
make run
```

### From sources

Requires: [Rust](https://www.rust-lang.org) and a running [Redis](https://redis.io/) instance.

```bash
export RUST_LOG=rust_captcha=info
export REDIS_HOST=localhost

git clone https://github.com/daniel-e/rust-captcha.git
cd rust-captcha
cargo run --release
```

*On Ubuntu 18.04 it can happen that openssl does not compile. Install libssl1.0-dev via `sudo apt install libssl1.0-dev` which will fix the problem.*

The service is listening on port 8080 for incoming requests.

If you don't have Redis already running type `make redis` in another console in the same directory. This command
will compile and execute Redis in the `target` directory.

# Interface

The service provides an API to create a new CAPTCHA and to check the
solution of a CAPTCHA.

## Create new CAPTCHA

With the following cURL a new CAPTCHA is created.
```bash
curl -s -i -XPOST -H "X-Client-ID: myclient" http://localhost:8080/new/<d>/<n>/<exp>
```

* A new CAPTCHA is created via a POST request.
* The header `X-Client-ID` is optional. It can be used to separate different clients using the same service instance when analyzing the service's logfile.
* `<d>`: The difficulty. Valid values are `easy`, `medium`, `hard`
* `<n>`: Maximum number of trials. Valid values are 0..999
* `<exp>`: Number of seconds after which the CAPTCHA expires. Valid values are 0..999

**Response**

On success "200 OK" is returned and the body of the response contains the
following JSON:

```
{
    "id": "75e41e21-e7be-4d6f-af1b-ce8f052dda7e"
    "png": "iVBORw0KGgoAAAANSUhEUgAAAN0AAAB5CAAAAACYIns+AAA..."
}
```

* `id`: The id of the CAPTCHA.
* `png`: The raw PNG image data encoded as base64.

**Errors**

* 500 Internal Server Error: internal error (e.g. no connection to Redis)
* 400 Bad Request: invalid parameters

## Check solution for a CAPTCHA

```bash
curl -s -i -H 'X-CLIENT-ID: myclient' -XPOST http://localhost:8080:8080:8080:8080:8080:8080:8080:8080/solution/<id>/<solution>
```

* The solution of the CAPTCHA is checked via a POST request.
* The header `X-Client-ID` is optional. It can be used to separate different clients using the same service instance when analyzing the service's logfile.
* `<id>`: The id of the CAPTCHA.
* `<solution>`: The CAPTCHA solution.

**Response**

If the service was able to process the request "200 OK" is returned and the body of the response contains the
following JSON:

```
{
    "result": "accepted" | "rejected",
    "reject_reason": "too many trials" | "incorrect solution" | "",
    "trials_left": <n>
}
```

* If the solution for the CAPTCHA with the given id is correct `result` is set to `accepted`. The `reject_reason` is empty and `trials_left` is set to `0`. The CAPTCHA is removed from the database so that the solution cannot be used again.
* If the solution for the CAPTCHA is incorrect `result` is set to `rejected` and `reject_reason` is set to one of the following values:
  * `too many trials`: The maximum attempts to solve the CAPTCHA exceeded.
  * `incorrect solution`: The solution is incorrect. The number of attempts left to solve the CAPTCHA is provided in `trials_left`.

**Errors**

* 500 Internal Server Error: internal error (e.g. no connection to Redis)
* 400 Bad Request: invalid parameters
* 404 Not Found: The CAPTCHA with the given id does not exist (e.g. because it has expired or a correct solution was presented in a previous request).
