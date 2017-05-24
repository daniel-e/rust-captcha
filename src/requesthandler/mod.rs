use rustful::{Handler, Context, Response, StatusCode};
use methods::{CaptchaError, CaptchaNewResult, CaptchaSolutionResult, captcha_new, captcha_solution};
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
        match self.method {
            CaptchaMethod::New => {
                match req_captcha_new(c) {
                    Ok(details) => {
                        info!("Created new CAPTCHA [{}].", details.uuid());
                        res.headers_mut().set(
                            ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)]))
                        );
                        res.send(details.as_json().as_str());
                    },
                    Err(e) => {
                        error!("Failed to create new CAPTCHA [{:?}].", e);
                        res.set_status(map_err(e))
                    }
                }
            },
            CaptchaMethod::Solution => {
                match req_captcha_solution(c) {
                    Ok(details) => {
                        info!("Solution checked for [{}] [{}].", details.uuid(), details.csr().result());
                        res.headers_mut().set(
                            ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)]))
                        );
                        res.send(details.as_json().as_str());
                    },
                    Err(e) => {
                        error!("Failed to check solution [{:?}].", e);
                        res.set_status(map_err(e))
                    }
                }
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

fn req_captcha_new(c: Context) -> CaptchaNewResult {
    captcha_new(val(&c, "difficulty")?, val(&c, "max_tries")?, val(&c, "ttl")?)
}

fn req_captcha_solution(c: Context) -> CaptchaSolutionResult {
    captcha_solution(val(&c, "id")?, val(&c, "solution")?)
}
