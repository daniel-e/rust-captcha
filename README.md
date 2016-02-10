# TODO

- [ ] generate the CAPTCHA
- [ ] maybe the persistence layer should not know anything about a CAPTCHA
- [ ] update documentation
- [ ] how to link against MagickWand properly?
- [x] compile c library for image creation with cargo

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

# Examples

## Create a new CAPTCHA

```
curl -i -X POST localhost:8080/session
```

**Response on success:**

```
HTTP/1.1 201 Created
Location: /session/WHilumBnJGMjOAReDA4u
...

{"solved":false,"tries":0,"max_tries":4}
```

On success the service returns with the status code 201 and the URI of the CAPTCHA in the location header field. The URI is used to verify a solution for the CAPTCHA or to retrieve the status of a CAPTCHA.

**Response on failure:**

```
HTTP/1.1 503 Service Unavailable
```

There are currently three possible reasons for the service to return with a status code 503:

* an instance of a random number generator could not be created (e.g. if /dev/urandom cannot be accessed)
* the service cannot connect to Redis
* the service was unable to store a CAPTCHA in Redis

## Get status of a CAPTCHA

```
curl -i -X GET localhost:8080/session/WHilumBnJGMjOAReDA4u
```

**Response on success:**

```
HTTP/1.1 200 OK
...

{"solved":false,"tries":0,"max_tries":4}
```

**Responses on failure:**

* `400 Bad Request`: Validation error.
* `404 Not Found`: CAPTCHA not found.
* `500 Internal Server Error`: Data from database could not be decoded.
* `503 Service Unavailable`: Connection to Redis failed.

--------------------------------------------------------------------------------

## Check the solution for a CAPTCHA

POST /session/:session

{
  solution: "..."
}

->

200 OK

{
  checked: true | false
  solved:
  tries: 1
  max_tries: 3
}

- if tries >= max_tries || solved:
  - status = "not_checked"
- else
  - status = "checked"
  - tries++
  - if solution is correct
    - solved = true
  - else
    - solved = false
