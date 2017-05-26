use rustful::{Handler, Context, Response, StatusCode};
use methods::{CaptchaError, captcha_new, captcha_solution};
use rustful::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

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
}

impl Handler for RequestHandler {
    fn handle_request(&self, c: Context, mut res: Response) {
        let r = match self.method {
            CaptchaMethod::New      => req_captcha_new(c),
            CaptchaMethod::Solution => req_captcha_solution(c)
        };
        match r {
            Err(e)   => res.set_status(map_err(e)),
            Ok(body) => {
                res.headers_mut().set(
                    ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)]))
                );
                res.send(body.as_str());
            }
        };
    }
}

fn map_err(e: CaptchaError) -> StatusCode {
    match e {
        CaptchaError::InvalidParameters => StatusCode::BadRequest,
        CaptchaError::CaptchaGeneration => StatusCode::InternalServerError,
        CaptchaError::Uuid              => StatusCode::InternalServerError,
        CaptchaError::ToJson            => StatusCode::InternalServerError,
        CaptchaError::Persist           => StatusCode::InternalServerError,
        CaptchaError::NotFound          => StatusCode::NotFound,
        CaptchaError::Unexpected        => StatusCode::InternalServerError
    }
}

fn val(c: &Context, k: &str) -> Result<String, CaptchaError> {
    Ok(c.variables.get(k).ok_or(CaptchaError::InvalidParameters)?.to_string())
}

fn req_captcha_new(c: Context) -> Result<String, CaptchaError> {
    match captcha_new(val(&c, "difficulty")?, val(&c, "max_tries")?, val(&c, "ttl")?) {
        Ok(details) => {
            info!("Created new CAPTCHA [{}].", details.uuid());
            Ok(details.as_json())
        },
        Err(e)      => {
            match e {
                CaptchaError::NotFound | CaptchaError::InvalidParameters => info!("Failed to create new CAPTCHA [{:?}].", e),
                _ => error!("Failed to create new CAPTCHA [{:?}].", e)
            }
            Err(e)
        }
    }
}

fn req_captcha_solution(c: Context) -> Result<String, CaptchaError> {
    match captcha_solution(val(&c, "id")?, val(&c, "solution")?) {
        Ok(details) => {
            info!("Solution checked for [{}] [{}] [{}].", details.uuid(), details.csr().result(), details.csr().reason());
            Ok(details.as_json())
        },
        Err(e)      => {
            match e {
                CaptchaError::NotFound | CaptchaError::InvalidParameters => info!("Failed to check solution [{:?}].", e),
                _ => error!("Failed to check solution [{:?}].", e)
            }
            Err(e)
        }
    }
}
