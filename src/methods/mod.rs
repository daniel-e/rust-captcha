use captcha::{Difficulty, gen};
use validation::*;
use persistence::persist;

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
}

pub fn captcha_new(difficulty: String, max_tries: String, ttl: String) -> CaptchaResult {

    let d = validate_difficulty(difficulty)?;
    let x = validate_tries(max_tries)?;
    let t = validate_ttl(ttl)?;

    let uuid = create_uuid()?;
    let (solution, png) = create_captcha(d)?;

    let c = NewCaptcha {
        id: uuid.clone(),
        png: encode(&png)
    };

    let json = serde_json::to_string(&c).map_err(|_| CaptchaError::ToJson)?;

    persist(uuid, solution, x, t)
        .map(|_| json)
        .map_err(|_| CaptchaError::Persist)
}

pub fn captcha_solution(id: String, solution: String) -> CaptchaResult {
    // TODO implement
    Ok("sol".to_string())
}

// -------------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct NewCaptcha {
    id: String,
    png: String
}

fn create_uuid() -> Result<String, CaptchaError> {
    Uuid::new(UuidVersion::Random).map(|x| x.hyphenated().to_string()).ok_or(CaptchaError::Uuid)
}

fn create_captcha(d: Difficulty) -> Result<(String, Vec<u8>), CaptchaError> {
    gen(d).as_tuple().ok_or(CaptchaError::CaptchaGeneration)
}
