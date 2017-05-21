* readme updaten
* tests
* docker
* types instead of strings

REDIS_HOST=localhost cargo run

curl -s -i -XPOST http://localhost:8080/new/easy/0/30
curl -s -i -XPOST http://localhost:8080/solution/59784187-68b7-44bd-9157-4c4e5a5b50f4/solution


# RESTful CAPTCHA Service

A RESTful CAPTCHA service written in Rust. The service generates images that can be embedded into web pages to protect them from being accessed by bots. The difficulty of the CAPTCHAs can be easily configured via a JSON file.

## Requirements

* Rust 1.15+
* On Ubuntu 16.10 the packages libmagickwand-dev libssl-dev redis-server.

To install the requirements type the following commands on the command line:

```bash
# Install additional packages on Ubuntu 16.10.
sudo apt-get -y install libmagickwand-dev libssl-dev redis-server
# Install the latest version of Rust.
# see: https://www.rust-lang.org/en-US/install.html
curl https://sh.rustup.rs -sSf | sh
```

## Running the service

Check out the sources:
```
git clone git@github.com:daniel-e/rust-captcha.git
```

Compile the sources:
```
cd rust-captcha
cargo build --release
```

Start Redis:

```
redis-server &
```

Start the service:
```
RUST_LOG=info ./target/release/rust-captcha -c config.json
```

Testing

Create a new image:

```
curl -s -X POST localhost:8080/session | jq -r .png_data | base64 -d | display
```


## Interfaces

Rust-CAPTCHA provides an interface to
* create a new CAPTCHA
* to get the status of a CAPTCHA
* to check a solution.
The different methods and parameters are summarized in the following table.

| Method | Path     | Input parameters | Output parameters                  | Description |
|--------|----------|------------------|------------------------------------|-------------|
| POST   | /session | -                | png_data, solved, tries, max_tries | Create new CAPTCHA. |
| GET    | /session/:sessionid | -     | png_data, solved, tries, max_tries | Get status of an existing CAPTCHA. |
| POST   | /session/:sessionid | solution | checked, info, solved, tries, max_tries | Check solution. |

Input and output parameters are provided in JSON in the body of the HTTP
request / response. The semantic of the different parameters is as follows:

| Parameter | Description |
|-----------|-------------|
| png_data | The image of the CAPTCHA encoded as a PNG image in base64. |
| solved   | Is true if the CAPTCHA has been solved. |
| tries    | Number of tries to solve the CAPTCHA. |
| max_tries | Maximum number of tries that are allowed to solved the CAPTCHA. |
| checked  | True if the provided solution has been checked. |
| info | Human readable message. |
| solution | Solution of the CAPTCHA. |

When a new CAPTCHA is created the response of the service will contain the
session id of the new CAPTCHA in the "Location" header field.

## Examples with curl

### Create a new CAPTCHA

```
curl -i -X POST localhost:8080/session
```

**Example response on success:**

```
HTTP/1.1 201 Created
Location: /session/WHilumBnJGMjOAReDA4u
...

{"png_data":"iVBORw0KGgo...",solved":false,"tries":0,"max_tries":4}
```

On success the service returns with the status code 201 and the URI of the CAPTCHA in the location header field. The URI is used to verify a solution for the CAPTCHA or to retrieve the status of a CAPTCHA. The field "png_data" contains the CAPTCHA image encoded as a PNG image in base64.

**Example response on failure:**

```
HTTP/1.1 500 Internal Server Error
```

On error the service returns with the status code "500 Internal Server Error". This error occurs if the service could not connect to the Redis database, the service was unable to store the CAPTCHA in the database or the image could not be created.

### Get the status of a CAPTCHA

```
curl -i -X GET localhost:8080/session/WHilumBnJGMjOAReDA4u
```

**Example response on success:**

```
HTTP/1.1 200 OK
...

{"png_data":"iVBORw0KGgo...","solved":false,"tries":0,"max_tries":4}
```

**Example response on failure:**

```
HTTP/1.1 500 Internal Server Error
```

On error the service returns with one of the following status code:

* `400 Bad Request`: Validation error.
* `404 Not Found`: CAPTCHA not found.
* `500 Internal Server Error`: Connection to Redis failed or data from database could not be decoded.

--------------------------------------------------------------------------------

### Check the solution for a CAPTCHA

```
curl -i -X POST -d '{"solution": "adasdf"}' localhost:8080/session/WHilumBnJGMjOAReDA4u
```

**Example response on success:**

```
HTTP/1.1 200 OK
...

{"checked":true,"info":"Incorrect.","solved":false,"tries":1,"max_tries":4}
```

The following checks are done in the following order:

* compare the number of tries with the number of maximum tries
* check if the CAPTCHA is already solved
* check the provided solution

Depending on the checks the values of the returned JSON are set accordingly.

| Check              | checked | info            | solved         | tries |
|--------------------|---------|-----------------|----------------|-------|
| tries >= max_tries | false   | Too many tries. | *not modified* | *not modified*   |
| already solved     | false   | Already solved. | *not modified* | *not modified*   |
| correct solution   | true    | Correct.        | true           | incremented by 1 |
| invalid solution   | true    | Incorrect.      | *not modified* | incremented by 1 |

**Example response on failure:**

```
HTTP/1.1 500 Internal Server Error
```

On error the service returns with one of the following status code:

* `400 Bad Request`: Validation error.
* `404 Not Found`: CAPTCHA not found.
* `500 Internal Server Error`: Connection to Redis failed or data from database could not be decoded.

## TODO

- [ ] list of fonts / which font to use
- [ ] maybe the persistence layer should not know anything about a CAPTCHA
- [ ] how to link against MagickWand properly?
- [ ] check min and max values in generator.rs:image()
- [ ] monitoring
