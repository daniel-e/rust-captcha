use std::str::FromStr;

use captcha::Difficulty;
use methods::CaptchaError;

use uuid::Uuid;

pub fn validate_difficulty(s: String) -> Result<Difficulty, CaptchaError> {
    match s.as_str() {
        "easy"   => { return Ok(Difficulty::Easy); },
        "medium" => { return Ok(Difficulty::Medium); },
        "hard"   => { return Ok(Difficulty::Hard); }
        _        => { return Err(CaptchaError::InvalidParameters); }
    }
}

pub fn validate_tries(s: String) -> Result<usize, CaptchaError> {
    if s.len() > 3 {
        return Err(CaptchaError::InvalidParameters);
    }
    s.parse::<usize>().map_err(|_| CaptchaError::InvalidParameters)
}

pub fn validate_ttl(s: String) -> Result<i64, CaptchaError> {
    Ok(validate_tries(s)? as i64)
}

pub fn validate_id(s: String) -> Result<Uuid, CaptchaError> {
    if s.len() > 60 {
        return Err(CaptchaError::InvalidParameters);
    }
    Uuid::from_str(s.as_str()).map_err(|_| CaptchaError::InvalidParameters)
}

pub fn validate_solution(s: String) -> Result<String, CaptchaError> {
    if s.len() > 10 {
        return Err(CaptchaError::InvalidParameters);
    }
    Ok(s)
}