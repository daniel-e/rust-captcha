use rustful::header::Location;
use rustful::{Context, Response, StatusCode};

use session::Session;
use executor::{create_and_persist_captcha, get_captcha, ExecutorError, check_captcha};
use config::Config;
use captchatools::{CaptchaToJson, CaptchaSolution, CaptchaSolutionConstraints};

use std::io::Read;

// Returns the following HTTP status codes:
// BadRequest           if session could not be extracted from request
// Ok                   if CAPTCHA was found
// NotFound             if CATPCHA does not exist
// InternalServerError  otherwise (e.g. connection to Redis failed)
pub fn req_get_catpcha(context: Context, config: Config) -> (String, StatusCode) {

    get_session(&context).map_or(bad_request(), |session| {
        match get_captcha(session, config) {
            Err(e) => map_error(e),
            Ok(c) => (c.to_json(), StatusCode::Ok)
        }
    })
}

pub fn req_check_solution(mut context: Context, config: Config) -> (String, StatusCode) {

    get_session(&context).map_or(bad_request(), |session| {
        let mut body = String::new();
        match context.body.read_to_string(&mut body) {
            Ok(_) => {
                match CaptchaSolution::from_json(body, CaptchaSolutionConstraints::new(&config)) {
                    Some(cs) => check_solution(session, cs, config),
                    None => bad_request()
                }
            },
            Err(_) => bad_request()
        }
    })
}

// Returns the following HTTP status codes:
// Created              if CAPTCHA has been created
// InternalServerError  otherwise (e.g. connection to Redis failed)
pub fn req_create_captcha(response: &mut Response, config: Config) -> (String, StatusCode) {

    match create_and_persist_captcha(config) {
        Ok(r) => {
            response.headers_mut().set(Location("/session/".to_string() + &r.session));
            (r.captcha.to_json(), StatusCode::Created)
        },
        Err(e) => map_error(e)
    }
}

// -----------------------------------------------------------------------------------------------

fn check_solution(id: Session, cs: CaptchaSolution, cf: Config) -> (String, StatusCode) {

    match check_captcha(id, cs, cf) {
        Ok(r) => (r.to_json(), StatusCode::Ok),
        Err(e) => map_error(e)
    }
}

/// Map an executor error to an HTTP status code.
fn map_error(e: ExecutorError) -> (String, StatusCode) {

    let code = match e {
        ExecutorError::ConnectionFailed => StatusCode::InternalServerError,
        ExecutorError::NotFound => StatusCode::NotFound,
        ExecutorError::JsonError => StatusCode::InternalServerError,
        ExecutorError::NoRng => StatusCode::InternalServerError,
        ExecutorError::DatabaseError => StatusCode::InternalServerError,
        ExecutorError::GeneratorFailed => StatusCode::InternalServerError,
    };

    (String::new(), code)
}

fn get_session(context: &Context) -> Option<Session> {
    context.variables.get("id").map_or(None, |id| {
        Session::from_string(id.to_string())
    })
}

fn bad_request() -> (String, StatusCode) {
    (String::new(), StatusCode::BadRequest)
}
