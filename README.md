* auto-get TOC:
{:toc}

# Examples

## Create a new CAPTCHA

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

## Get the status of a CAPTCHA

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

## Check the solution for a CAPTCHA

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

# TODO

- [ ] list of fonts / which font to use
- [ ] maybe the persistence layer should not know anything about a CAPTCHA
- [ ] how to link against MagickWand properly?
- [ ] check min and max values in generator.rs:image()
- [x] encode raw pixels into png and write this data into a buffer in memory
- [x] generate the CAPTCHA
- [x] compile c library for image creation with cargo
- [x] update documentation
- [x] fixed warnings about unused functions, structs, etc
- [x] fix version numbers in Cargo.toml
- [x] create build.rs to check for MagickWand dependencies
- [x] implement GET request to retrieve status of a CAPTCHA
- [x] Makefile to start redis instance on starting the service
- [x] configure redis endpoint in configuration file
- [x] what is the response if the CAPTCHA does not exist for a GET?
- [x] configure redis port
- [x] configure TTL for entries in redis in configuration file
- [x] configure port for server in configuration file
- [x] filter log messages
- [x] implement POST request to solve a CAPTCHA
