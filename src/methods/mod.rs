use captcha::{Difficulty, gen};
use validation::*;
use persistence::{Persistence, Item, Error, build_item};

use uuid::{Uuid, UuidVersion};
use base64::encode;
use serde_json;
use time;

pub type CaptchaResult = Result<String, CaptchaError>;

#[derive(Debug)]
pub enum CaptchaError {
    InvalidParameters,
    CaptchaGeneration,
    Uuid,
    ToJson,
    Persist,
    NotFound,
    Unexpected
}

pub fn captcha_new(difficulty: String, max_tries: String, ttl: String) -> CaptchaResult {

    let d = validate_difficulty(difficulty)?;
    let x = validate_tries(max_tries)?;
    let t = validate_ttl(ttl)?;

    let uuid = create_uuid()?;
    let (solution, png) = create_captcha(d)?;

    let c = NewCaptchaResponse {
        id: uuid.clone(),
        png: encode(&png)
    };

    let json = serde_json::to_string(&c).map_err(|_| CaptchaError::ToJson)?;

    let item = build_item()
        .uuid(uuid)
        .solution(solution)
        .tries_left(x)
        .ttl(t)
        .item()
        .map_err(|_| CaptchaError::Unexpected)?;

    info!("new {:?}", item);
    Persistence::set(item)
        .map(|_| json)
        .map_err(|_| CaptchaError::Persist)
}

pub fn captcha_solution(id: String, solution: String) -> CaptchaResult {

    let i = validate_id(id)?;
    let s = validate_solution(solution)?;

    Persistence::get(i.hyphenated().to_string())
        .map_err(persistence_error_mapping)
        .and_then(|item| check(s, item))
}

// -------------------------------------------------------------------------------------------------

fn persistence_error_mapping(e: Error) -> CaptchaError {
    match e {
        Error::Connection |
        Error::NoLocation  => CaptchaError::Persist,
        Error::NotFound    => CaptchaError::NotFound,
        Error::Json        => CaptchaError::Persist
    }
}

fn check_solution(user_solution: String, item: Item) -> CaptchaSolutionResponse {
    if item.solution() == user_solution {
        Persistence::del(item.uuid());
        CaptchaSolutionResponse::accept()
    } else {
        let t = time::now().to_timespec().sec;
        if item.expires() > t {
            Persistence::set(item.dec_tries_left()).ok();
        } else {
            Persistence::del(item.uuid());
        };
        CaptchaSolutionResponse::reject("incorrect solution", item.tries_left() - 1)
    }
}

fn check(user_solution: String, item: Item) -> CaptchaResult {
    info!("tries: {} {}", user_solution, item.tries_left());
    let r = match item.tries_left() {
        0 => CaptchaSolutionResponse::reject("too many trials", 0),
        _ => check_solution(user_solution, item)
    };
    Ok(serde_json::to_string(&r).map_err(|_| CaptchaError::ToJson)?)
}

#[derive(Serialize)]
struct CaptchaSolutionResponse {
    result: String,
    reject_reason: String,
    trials_left: usize
}

impl CaptchaSolutionResponse {
    pub fn reject(reason: &str, trials_left: usize) -> CaptchaSolutionResponse {
        CaptchaSolutionResponse {
            result: String::from("rejected"),
            reject_reason: reason.to_string(),
            trials_left: trials_left
        }
    }

    pub fn accept() -> CaptchaSolutionResponse {
        CaptchaSolutionResponse {
            result: String::from("accepted"),
            reject_reason: String::new(),
            trials_left: 0
        }
    }
}

#[derive(Serialize)]
struct NewCaptchaResponse {
    id: String,
    png: String
}

fn create_uuid() -> Result<String, CaptchaError> {
    Uuid::new(UuidVersion::Random).map(|x| x.hyphenated().to_string()).ok_or(CaptchaError::Uuid)
}

fn create_captcha(d: Difficulty) -> Result<(String, Vec<u8>), CaptchaError> {
    gen(d).as_tuple().ok_or(CaptchaError::CaptchaGeneration)
}
