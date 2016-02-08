# TODO

- [x] Makefile to start redis instance on starting the service
- [ ] configure TTL for entries in redis in configuration file
- [ ] configure redis endpoint in configuration file
- [ ] filter log messages
- [ ] what is the response if the CAPTCHA does not exist for a GET?
- [ ] configure port for server in configuration file
- [ ] generate the CAPTCHA
- [ ] implement GET request to retrieve status of a CAPTCHA
- [ ] implement POST request to solve a CAPTCHA

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




--------------------------------------------------------------------------------

## Get a created CAPTCHA

GET /session/:session

->

200 OK
Content-type: text/json

{
  imagedata: "..."
  tries: 0
  max_tries: 3
  status: "processed" | "rejected"
  solved:
}

--------------------------------------------------------------------------------

## Check the solution for a CAPTCHA

POST /session/:session

{
  solution: "..."
}

->

200 OK

{
  status: "processed" | "rejected"
  solved:
  tries: 1
  max_tries: 3
}

- if tries >= max_tries:
  - status = "rejected"
- else if solved
  - status = "rejected"
- else
  - tries++
  - if solution is correct
    - status = "processed"
    - solved = true
  - else
    - status = "processed"
    - solved = false
