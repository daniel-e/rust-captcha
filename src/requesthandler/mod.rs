use rustful::{Handler, Context, Response, StatusCode};
use methods::{CaptchaError, CaptchaResult, captcha_new, captcha_solution};

pub enum CaptchaMethod {
    New,
    Solution
}

pub struct RequestHandler {
    method: CaptchaMethod
}

impl RequestHandler {
    pub fn new(c: CaptchaMethod) -> RequestHandler {
        RequestHandler {
            method: c,
        }
    }

    fn captcha_new(c: Context) -> CaptchaResult {
        let d = c.variables.get("difficulty");
        let t = c.variables.get("max_tries");
        let l = c.variables.get("ttl");

        if d.clone().and(t.clone().and(l.clone())).is_none() {
            return Err(CaptchaError::InvalidParameters);
        }

        captcha_new(d.unwrap().to_string(), t.unwrap().to_string(), l.unwrap().to_string())
    }

    fn captcha_solution(c: Context) -> CaptchaResult {
        let i = c.variables.get("id");
        let s = c.variables.get("solution");

        if i.clone().and(s.clone()).is_none() {
            return Err(CaptchaError::InvalidParameters);
        }

        captcha_solution(i.unwrap().to_string(), s.unwrap().to_string())
    }

    fn check(r: CaptchaResult, mut res: Response) {
        match r {
            Err(e) => {
                match e {
                    CaptchaError::InvalidParameters => res.set_status(StatusCode::BadRequest),
                    CaptchaError::CaptchaGeneration => res.set_status(StatusCode::InternalServerError),
                    CaptchaError::Uuid              => res.set_status(StatusCode::InternalServerError),
                    CaptchaError::ToJson            => res.set_status(StatusCode::InternalServerError),
                    CaptchaError::Persist           => res.set_status(StatusCode::InternalServerError),
                }
            },
            Ok(s) => {
                res.send(s.as_str());
            }
        }
    }
}

impl Handler for RequestHandler {
    fn handle_request(&self, c: Context, res: Response) {
        match self.method {
            CaptchaMethod::New => Self::check(Self::captcha_new(c), res),
            CaptchaMethod::Solution => Self::check(Self::captcha_solution(c), res)
        }
    }
}

