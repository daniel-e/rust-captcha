use captcha::Difficulty;
use methods::CaptchaError;

pub fn validate_difficulty(s: String) -> Result<Difficulty, CaptchaError> {
    match s.as_str() {
        "easy"   => { return Ok(Difficulty::Easy); },
        "medium" => { return Ok(Difficulty::Medium); },
        "hard"   => { return Ok(Difficulty::Hard); }
        _        => { return Err(CaptchaError::InvalidParameters); }
    }
}

pub fn validate_tries(s: String) -> Result<u32, CaptchaError> {
    if s.len() > 3 {
        return Err(CaptchaError::InvalidParameters);
    }

    match s.parse::<u32>() {
        Ok(n) => { return Ok(n); },
        _     => { return Err(CaptchaError::InvalidParameters); }
    }
}

pub fn validate_ttl(s: String) -> Result<u32, CaptchaError> {
    validate_tries(s)
}