extern crate rand;

use self::rand::os::OsRng;
use self::rand::Rng;
use std::io::Read;
use super::rustc_serialize::json;

use config::Config;

pub trait CaptchaToJson: Sized {
    fn to_json(&self) -> String;
}

// ----------------------------------------------------------------------------

#[derive(RustcDecodable, RustcEncodable)]
pub struct CaptchaSolutionResponse {
    pub checked: bool,
    pub info: String,
    pub solved: bool,
    pub tries: usize,
    pub max_tries: usize,
}

impl CaptchaSolutionResponse {
    pub fn new(c: &Captcha) -> CaptchaSolutionResponse {
        CaptchaSolutionResponse {
            checked: false,
            info: "".to_string(),
            solved: c.solved,
            tries: c.tries,
            max_tries: c.max_tries
        }
    }

    pub fn set_reason(self, reason: &str) -> CaptchaSolutionResponse {
        CaptchaSolutionResponse {
            info: reason.to_string(), .. self
        }
    }

    pub fn set_checked(self, val: bool) -> CaptchaSolutionResponse {
        CaptchaSolutionResponse {
            checked: val, .. self
        }
    }

    pub fn inc_tries(self) -> CaptchaSolutionResponse {
        CaptchaSolutionResponse {
            tries: self.tries + 1, .. self
        }
    }

    pub fn set_solved(self, val: bool) -> CaptchaSolutionResponse {
        CaptchaSolutionResponse {
            solved: val, .. self
        }
    }
}

impl CaptchaToJson for CaptchaSolutionResponse {
    fn to_json(&self) -> String {
        json::encode(self).unwrap() // TODO error handling
    }
}

// ----------------------------------------------------------------------------

#[derive(RustcDecodable, RustcEncodable)]
pub struct CaptchaCreation {
    solved: bool,
    tries: usize,
    max_tries: usize,
}

impl CaptchaCreation {
    pub fn new(c: Captcha) -> CaptchaCreation {
        CaptchaCreation {
            solved: c.solved,
            tries: c.tries,
            max_tries: c.max_tries,
        }
    }
}

impl CaptchaToJson for CaptchaCreation {
    fn to_json(&self) -> String {
        json::encode(self).unwrap() // TODO error handling
    }
}

// ----------------------------------------------------------------------------

#[derive(RustcDecodable, RustcEncodable)]
pub struct Captcha {
    pub solution: String,
    pub tries: usize,
    pub max_tries: usize,
    pub session: String,
    pub solved: bool,
}

impl Captcha {
    pub fn new(c: &CaptchaSolutionResponse, session: &String, solution: &String) -> Captcha {
        Captcha {
            solution: solution.clone(),
            tries: c.tries,
            max_tries: c.max_tries,
            session: session.clone(),
            solved: c.solved,
        }
    }

    pub fn from_json(s: String) -> Option<Captcha> {
        match json::decode(&s) {
            Ok(c)  => Some(c),
            Err(_) => None
        }
    }
}

impl CaptchaToJson for Captcha {
    fn to_json(&self) -> String {
        json::encode(self).unwrap() // TODO
    }
}

// ----------------------------------------------------------------------------

pub struct CaptchaSolutionConstraints {
    pub minlen: usize,
    pub maxlen: usize,
    pub chars: String,
}

impl CaptchaSolutionConstraints {
    pub fn new(conf: &Config) -> CaptchaSolutionConstraints {
        CaptchaSolutionConstraints {
            minlen: conf.min_letters,
            maxlen: conf.max_letters,
            chars: conf.characters.clone(),
        }
    }
}
// ----------------------------------------------------------------------------

#[derive(RustcDecodable, RustcEncodable)]
pub struct CaptchaSolution {
    pub solution: String,
}

impl CaptchaSolution {

    pub fn new(constraints: CaptchaSolutionConstraints) -> Option<CaptchaSolution> {

        let minlen = constraints.minlen;
        let maxlen = constraints.maxlen;

        OsRng::new().ok().and_then(|mut rng| {
            let r = rng.next_u32() as usize;
            let l = r % (maxlen - minlen + 1) + minlen;
            let c = constraints.chars.chars().collect::<Vec<_>>();

            let mut s = String::new();
            for _ in 0..l {
                s.push(*rng.choose(&c).unwrap());
            }
            Some(CaptchaSolution { solution: s })
        })
    }

    fn validate(self, constraints: CaptchaSolutionConstraints) -> Option<CaptchaSolution> {

        let minlen = constraints.minlen;
        let maxlen = constraints.maxlen;

        let l = self.solution.chars().count();
        let r = l >= minlen && l <= maxlen && self.solution.chars().all(|x| constraints.chars.contains(x));
        match r {
            true  => Some(self),
            false => None
        }
    }

    pub fn from_json(s: String, constraints: CaptchaSolutionConstraints) -> Option<CaptchaSolution> {
        let r: Result<CaptchaSolution, _> = json::decode(&s);
        match r {
            Ok(c)  => c.validate(constraints),
            Err(_) => None
        }
    }

    pub fn to_string(&self) -> String {
        self.solution.to_string()
    }
}

impl CaptchaToJson for CaptchaSolution {
    fn to_json(&self) -> String {
        json::encode(self).unwrap() // TODO
    }
}
