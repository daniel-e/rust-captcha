use captcha::{Difficulty, gen};
use validation::*;
use persistence::{Persistence, Item, Error, build_item};

use uuid::Uuid;
use base64::encode;
use serde_json;
use time;

pub type CaptchaNewResult = Result<CaptchaNewDetails, CaptchaError>;
pub type CaptchaSolutionResult = Result<CaptchaSolutionDetails, CaptchaError>;

pub struct CaptchaSolutionDetails {
    json: String,
    uuid: String,
    csr: CaptchaSolutionResponse,
}

impl CaptchaSolutionDetails {
    pub fn as_json(&self) -> String {
        self.json.clone()
    }

    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn csr(&self) -> CaptchaSolutionResponse {
        self.csr.clone()
    }
}

pub struct CaptchaNewDetails {
    json: String,
    uuid: String,
}

impl CaptchaNewDetails {
    pub fn as_json(&self) -> String {
        self.json.clone()
    }

    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }
}

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

pub fn captcha_new(difficulty: String, max_tries: String, ttl: String) -> CaptchaNewResult {

    let d = validate_difficulty(difficulty)?;
    let x = validate_tries(max_tries)?;
    let t = validate_ttl(ttl)?;

    let uuid = create_uuid()?;
    let (solution, png) = create_captcha(d)?;

    let c = NewCaptchaResponse {
        id: uuid.clone(),
        png: encode(&png)
    };

    let captcha = CaptchaNewDetails {
        json: serde_json::to_string(&c).map_err(|_| CaptchaError::ToJson)?,
        uuid: uuid.clone()
    };

    let item = build_item()
        .uuid(uuid)
        .solution(solution)
        .tries_left(x)
        .ttl(t)
        .item()
        .map_err(|_| CaptchaError::Unexpected)?;

    Persistence::set(item)
        .map(|_| captcha)
        .map_err(|_| CaptchaError::Persist)
}

pub fn captcha_solution(id: String, solution: String) -> CaptchaSolutionResult {

    let i = validate_id(id)?;
    let s = validate_solution(solution)?;

    let csr = Persistence::get(i.to_hyphenated().to_string())
        .map_err(persistence_error_mapping)
        .and_then(|item| check(s, item))?;

    let json = serde_json::to_string(&csr).map_err(|_| CaptchaError::ToJson)?;

    Ok(CaptchaSolutionDetails {
        json: json,
        uuid: i.to_hyphenated().to_string(),
        csr: csr
    })
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

fn check(user_solution: String, item: Item) -> Result<CaptchaSolutionResponse, CaptchaError> {
   Ok(match item.tries_left() {
        0 => CaptchaSolutionResponse::reject("too many trials", 0),
        _ => check_solution(user_solution, item)
    })
}

#[derive(Serialize, Clone)]
pub struct CaptchaSolutionResponse {
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

    pub fn result(&self) -> String {
        self.result.clone()
    }

    pub fn reason(&self) -> String {
        self.reject_reason.clone()
    }
}

#[derive(Serialize)]
struct NewCaptchaResponse {
    id: String,
    png: String
}

fn create_uuid() -> Result<String, CaptchaError> {
    Ok(Uuid::new_v4().to_hyphenated().to_string())
}

fn create_captcha(d: Difficulty) -> Result<(String, Vec<u8>), CaptchaError> {
    gen(d).as_tuple().ok_or(CaptchaError::CaptchaGeneration)
}
