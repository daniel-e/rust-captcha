[![Build Status](https://travis-ci.org/daniel-e/rust-captcha.svg?branch=master)](https://travis-ci.org/daniel-e/rust-captcha)

# Rust CAPTCHA Service

Here you find a CAPTCHA service written in Rust. The service generates CAPTCHAs
which can be embedded into web pages to protect them from being
accessed by bots. The difficulty of the CAPTCHAs, the expiration time and
the maximum number of attempts to solve a CAPTCHA can be easily
configured for each created CAPTCHA.

The CAPTCHAs look similar to the following ones:

![captcha](doc/captcha3.png) &nbsp; ![captcha](doc/captcha2.png) &nbsp; ![captcha](doc/captcha_mila_medium.png)

**The CAPTCHA service uses Rocket in version 0.4.5 which is a web framework to create fast and secure web applications. As Rocket requires Rust nightly, you also need Rust nightly to compile the CAPTCHA service.**

## Running

There are two ways how you can run the service.

### In Docker

```bash
cd docker
make rebuild   # build image; needs to be executed for the first time only
make run
```

### From sources

**Requirements**

To build from sources, you require [Rust](https://www.rust-lang.org) nightly and a running [Redis](https://redis.io/) instance.

* If you're using rustup, you can switch to Rust nightly by running `rustup default nightly`. Then, compile and run the CAPTCHA service as follows:
* If you don't have Redis already running, execute `make redis`. This command will compile and execute Redis in the `target` directory. 

**Build**

```bash
export RUST_LOG=rust_captcha=info
export REDIS_HOST=localhost

git clone https://github.com/daniel-e/rust-captcha.git
cd rust-captcha
cargo run --release
```

The service is listening on port 8080 for incoming requests.



# Usage

The service provides an API to create new CAPTCHAs and to check the solution of a CAPTCHA.

## Create new CAPTCHA without persisting the CAPTCHA

```bash
curl -s -i http://localhost:8000/new/<difficulty>
```

* `<difficulty>`: The difficulty. Valid values are `easy`, `medium`, `hard`.
* Optionally, you can provide a `X-Client-ID` header. This header can be used to separate different clients using the same service instance when analyzing the service's logfile.

**Response**

Example of a successful request:

```
{
  "code": 0,
  "msg": "processed",
  "result": {
    "id": "04f498ec-ad36-42f1-a56f-3cf5b9f912b3",
    "png": "<base64 encoded image>",
    "solution": "uS6c"
  }
}
```

* `error_code`: The error code. 
  * 0 = request was processed without error
  * 1 = internal error
  * 2 = invalid parameters were provided
* `error_msg`: The string representation of the error code. Can be 'processed', 'internal error' or 'invalid parameters'.
* `id`: The id of the CAPTCHA. For CAPTCHAs that are not persisted this field can be ignored.
* `png`: The raw PNG image data encoded as base64.
* `solution`: The solution.

## Create new CAPTCHA that is persisted

```bash
curl -s -i -XPOST http://localhost:8000/new/<difficulty>/<max_tries>/<ttl>
```

* `<difficulty>`: The difficulty. Valid values are `easy`, `medium`, `hard`.
* `<max_tries>`: Maximum number of trials. Valid values are 0..999
* `<ttl>`: Number of seconds after which the CAPTCHA expires. Valid values are 0..999
* Optionally, you can provide a `X-Client-ID` header. (see above)

**Response**

See request above.

## Check solution for a CAPTCHA

Solutions can only be checked for CAPTCHAs that have been created via a POST request.

```bash
curl -s -i -XPOST http://localhost:8000/solution/<id>/<solution>
```

* `<id>`: The id of the CAPTCHA.
* `<solution>`: The CAPTCHA solution.

**Response**

Example of a response of a correct solution:

```
{
  "error_code": 0,
  "error_msg": "processed",
  "result": {
    "solution": "accepted",
    "trials_left": 0
  }
}
```

* `error_code`: The error code. 0 = request was processed without error, 1 = internal error, 2 = invalid parameters were provided
* `error_msg`: The string representation of the error code. Can be 'processed', 'internal error' or 'invalid parameters'.
* `solution`: Contains the result of the check. Possible values are: 'too many trials', 'accepted' 'incorrect' or 'not found'
* `trials_left`: Number of attempts left to solve the CAPTCHA.
