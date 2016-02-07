RUST_LOG=debug cargo run -- -c config.json
curl -i -X POST localhost:8080/sessioncurl -i -X POST localhost:8080/session

- redis?
- ttl
- response if session does not exist

## Create a new CAPTCHA

POST /session

->

201 Created
Location: /session/:session/
Content-type: text/json

{
  imagedata: "..."
  tries: 0
  max_tries: 3
  solved: true
}

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
