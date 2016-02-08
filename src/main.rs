#[macro_use] extern crate rustful;
extern crate redis;

// see issue: https://github.com/rust-lang-nursery/rustc-serialize/issues/61
extern crate rustc_serialize;

#[macro_use] extern crate log;
extern crate env_logger;
extern crate getopts;

use std::error::Error;

use rustful::{Server, Context, Response, TreeRouter, Method, StatusCode};
use rustful::header::{ContentType, Location};

mod arguments;
mod executor;
mod config;
mod persistence;
mod captcha;

use arguments::parse_arguments;
use executor::{create_and_persist_captcha, validate_session};
use config::{parse_config, Config};
use captcha::CaptchaToJson;

fn bad_request() -> (String, StatusCode) {
    (String::new(), StatusCode::BadRequest)
}

/// Returns
/// - 201 if CAPTCHA has been created and persisted
/// - 503 if Redis is not available or key could not be stored
fn do_post(_: Context, response: &mut Response, config: &Config) -> (String, StatusCode) {

    match create_and_persist_captcha(config) {
        Some(r) => {
            let c = r.captcha;
            let s = r.session;
            let l = Location("/session/".to_string() + &s);
            response.headers_mut().set(l);
            (c.to_json(), StatusCode::Created)
        },
        _ => (String::new(), StatusCode::ServiceUnavailable)
    }
}

fn do_get(context: Context, config: &Config) -> (String, StatusCode) {

    match context.variables.get("id") {
        Some(id) => {
            let i = id.to_string();
            if !validate_session(&i) {
                warn!(target: "main::do_get", "Validation of id failed.");
                return bad_request();
            }
            info!(target: "main::do_get", "Got request for id [{}]", i);
            (String::new(), StatusCode::Ok)
            /*
            match get_captcha(id) {
                Ok(c)  => (c.to_json(), StatusCode::Ok),
                Err(e) => {
                    match e {

                    }
                }
            }
            */
        },
        None => {
            // This cannot happen.
            info!(target: "main::do_get", "Got request without an id.");
            bad_request()
        }
    }
}

fn do_request(context: Context, mut response: Response, config: &Config) {

    let (body, status) = match context.method {
        Method::Post => do_post(context, &mut response, config),
        Method::Get  => do_get(context, config),
        _            => bad_request(),
    };

    response
        .headers_mut()
        .set(ContentType(content_type!(Application / Json; Charset = Utf8)));
    response.set_status(status);
    response.send(body);
}

struct Handler(fn(Context, Response, &Config), Config);

impl rustful::Handler for Handler {
    fn handle_request(&self, context: Context, response: Response) {
        self.0(context, response, &self.1)
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

  info!(target: "main", "Starting server ...");

  let srv = Server {
    host: 8080.into(),
    handlers: insert_routes! {
      TreeRouter::new() => {
        "/session" => Post: Handler(do_request, conf.clone()),
        "/session/:id" => Get: Handler(do_request, conf.clone()),
      }
    },
    ..Server::default() // for the rest use default values
  }.run();

  match srv {
    Ok(_)  => { },
    Err(e) => { error!("Could not start server: {}", e.description()) }
  }
}
