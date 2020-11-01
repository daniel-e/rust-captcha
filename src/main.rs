#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate log;
#[macro_use] extern crate rocket;
extern crate env_logger;
extern crate rust_captcha;
extern crate serde_json;

use std::env;

use rust_captcha::requesthandler::{req_captcha_new, req_captcha_newget, req_captcha_solution};
use rust_captcha::methods::CaptchaError;
use rocket::response::content;
use serde_json::{json, Value};

const PORT: u16 = 8000;

fn precondition_checks() -> bool {
    match env::var("REDIS_HOST") {
        Err(_) => {
            error!("Environment variable REDIS_HOST not set.");
            false
        },
        Ok(_)  => true
    }
}

fn create_response(r: Result<String, CaptchaError>) -> content::Json<String> {
    let msg = vec!["OK", "Bad request", "Internal error", "Not found"];
    let ret = match r {
        Err(e) => {
            let code = match e {
                CaptchaError::InvalidParameters => 1,
                CaptchaError::CaptchaGeneration => 2,
                CaptchaError::Uuid => 2,
                CaptchaError::ToJson => 2,
                CaptchaError::Persist => 2,
                CaptchaError::NotFound => 3,
                CaptchaError::Unexpected => 2
            };
            json!({
                "code": code,
                "msg": msg[code],
                "result": ""
            })
        },
        Ok(json) => {
            let data: Value = serde_json::from_str(&json).unwrap();
            json!({
                "code": 0,
                "msg": msg[0],
                "result": data
            })
        }
    };

    content::Json(ret.to_string())
}

#[post("/new/<difficulty>/<max_tries>/<ttl>")]
fn new(difficulty: String, max_tries: String, ttl: String) -> content::Json<String> {
    create_response(req_captcha_new(difficulty, max_tries, ttl))
}

#[get("/new/<difficulty>")]
fn new_diff_only(difficulty: String) -> content::Json<String> {
    create_response(req_captcha_newget(difficulty))
}

#[post("/solution/<id>/<solution>")]
fn solution(id: String, solution: String) -> content::Json<String> {
    create_response(req_captcha_solution(id, solution))
}


fn main() {
    env_logger::init();

    if !precondition_checks() {
        error!("Failed to start server.");
        return;
    }

    info!("Starting service on port {} ...", PORT);
    rocket::ignite()
        .mount("/", routes![new, new_diff_only, solution])
        .launch();
}
