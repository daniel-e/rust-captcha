use captcha::{Difficulty, gen};
use validation::*;
use persistence::{persist, from_persistence, PersistenceError};

use uuid::{Uuid, UuidVersion};
use base64::encode;
use serde_json;

pub type CaptchaResult = Result<String, CaptchaError>;


pub enum CaptchaError {
    InvalidParameters,
    CaptchaGeneration,
    Uuid,
    ToJson,
    Persist,
    NotFound,
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

    persist(uuid, solution, x, t)
        .map(|_| json)
        .map_err(|_| CaptchaError::Persist)
}

pub fn captcha_solution(id: String, solution: String) -> CaptchaResult {

    let i = validate_id(id)?;
    let s = validate_solution(solution)?;

    from_persistence(i.hyphenated().to_string())
        .map_err(persistence_error_mapping)
        .and_then(|(solution_in_db, tries_left)| check(s, solution_in_db, tries_left))
}

// -------------------------------------------------------------------------------------------------

fn persistence_error_mapping(e: PersistenceError) -> CaptchaError {
    match e {
        PersistenceError::Connection |
        PersistenceError::NoLocation  => CaptchaError::Persist,
        PersistenceError::NotFound    => CaptchaError::NotFound,
        PersistenceError::InvalidData => CaptchaError::Persist
    }
}

fn check_solution(user_solution: String, db_solution: String, tries_left: u32) -> CaptchaSolutionResponse {
    if db_solution == user_solution {
        CaptchaSolutionResponse::accept()
        // TODO remove from redis
    } else {
        // TODO decrement
        CaptchaSolutionResponse::reject("incorrect solution", tries_left - 1)
    }
}

fn check(user_solution: String, db_solution: String, tries_left: u32) -> CaptchaResult {
    let r = match tries_left {
        0 => CaptchaSolutionResponse::reject("too many trials", 0),
        _ => check_solution(user_solution, db_solution, tries_left)
    };
    Ok(serde_json::to_string(&r).map_err(|_| CaptchaError::ToJson)?)
}

#[derive(Serialize)]
struct CaptchaSolutionResponse {
    result: String,
    reject_reason: String,
    trials_left: u32
}

impl CaptchaSolutionResponse {
    pub fn reject(reason: &str, trials_left: u32) -> CaptchaSolutionResponse {
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
