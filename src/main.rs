#[macro_use] extern crate log;
extern crate env_logger;
extern crate getopts;
// see issue: https://github.com/rust-lang-nursery/rustc-serialize/issues/61
extern crate rustc_serialize;

use std::error::Error;
use std::io::Read;

#[macro_use] extern crate rustful;
extern crate redis;

use rustful::{Server, Context, Response, TreeRouter, StatusCode};
use rustful::header::{ContentType, Location};

mod arguments;
mod executor;
mod config;
mod persistence;
mod captcha;
mod session;
mod image;
mod generator;

use arguments::parse_arguments;
use executor::{create_and_persist_captcha, get_captcha, ExecutorError, check_captcha};
use config::{parse_config, Config};
use captcha::{CaptchaToJson, CaptchaSolution, CaptchaSolutionConstraints};
use session::Session;

#[derive(Clone, Copy)]
enum RequestType {
    CreateCaptcha,
    GetCaptcha,
    CheckSolution,
}

fn check_solution(id: Session, cs: CaptchaSolution, cf: Config) -> (String, StatusCode) {

    match check_captcha(id, cs, cf) {
        Ok(r) => (r.to_json(), StatusCode::Ok),
        Err(e) => map_error(e)
    }
}

/// Map an executor error to an HTTP status code.
fn map_error(e: ExecutorError) -> (String, StatusCode) {

    let code = match e {
        ExecutorError::ConnectionFailed => StatusCode::ServiceUnavailable,
        ExecutorError::NotFound => StatusCode::NotFound,
        ExecutorError::JsonError => StatusCode::InternalServerError,
        ExecutorError::NoRng => StatusCode::ServiceUnavailable,
        ExecutorError::DatabaseError => StatusCode::InternalServerError,
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

fn req_get_catpcha(context: Context, config: Config) -> (String, StatusCode) {

    get_session(&context).map_or(bad_request(), |session| {
        match get_captcha(session, config) {
            Err(e) => map_error(e),
            Ok(c) => (c.to_json(), StatusCode::Ok)
        }
    })
}

fn req_check_solution(mut context: Context, config: Config) -> (String, StatusCode) {

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

fn req_create_captcha(response: &mut Response, config: Config) -> (String, StatusCode) {

    match create_and_persist_captcha(config) {
        Ok(r) => {
            response.headers_mut().set(Location("/session/".to_string() + &r.session));
            (r.captcha.to_json(), StatusCode::Created)
        },
        Err(e) => map_error(e)
    }
}

fn do_request(context: Context, mut response: Response, config: Config, rt: RequestType) {

    let (body, status) = match rt {
        RequestType::CreateCaptcha => req_create_captcha(&mut response, config),
        RequestType::GetCaptcha => req_get_catpcha(context, config),
        RequestType::CheckSolution => req_check_solution(context, config),
    };

    response
        .headers_mut()
        .set(ContentType(content_type!(Application / Json; Charset = Utf8)));
    response.set_status(status);
    response.send(body);
}

struct Handler(Config, RequestType);

impl rustful::Handler for Handler {
    fn handle_request(&self, context: Context, response: Response) {
        do_request(context, response, self.0.clone(), self.1)
    }
}

fn main() {
    env_logger::init().unwrap();

    let args = match parse_arguments() {
        Some(a) => a,
        _ => { return; }
    };

    let conf = match parse_config(&args.config_file) {
        Ok(a) => a,
        Err(msg) => {
            println!("Could not read configuration file: {}", msg);
            return;
        }
    };

    info!(target: "main", "Starting server on port {} ...", conf.port);

    let srv = Server {
        host: conf.port.into(),
        handlers: insert_routes! {
            TreeRouter::new() => {
                "/session" => Post: Handler(conf.clone(), RequestType::CreateCaptcha),
                "/session/:id" => Get: Handler(conf.clone(), RequestType::GetCaptcha),
                "/session/:id" => Post: Handler(conf.clone(), RequestType::CheckSolution),
            }
        },
        ..Server::default() // for the rest use default values
    }.run();

    match srv {
        Ok(_)  => { },
        Err(e) => { error!("Could not start server: {}", e.description()) }
    }
}
