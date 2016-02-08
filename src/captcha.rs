use super::rustc_serialize::json;

pub trait CaptchaToJson: Sized {
    fn to_json(&self) -> String;
}

// ----------------------------------------------------------------------------

#[derive(RustcDecodable, RustcEncodable)]
pub struct CaptchaCreation {
    solved: bool,
    tries: u32,
    max_tries: u32,
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
    pub tries: u32,
    pub max_tries: u32,
    pub session: String,
    pub solved: bool,
}

impl Captcha {
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
