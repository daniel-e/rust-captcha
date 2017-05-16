#[macro_use] extern crate log;
extern crate env_logger;
extern crate getopts;
// see issue: https://github.com/rust-lang-nursery/rustc-serialize/issues/61
extern crate rustc_serialize;

extern crate redis;
#[macro_use] extern crate rustful;
extern crate captcha;

use std::error::Error;

use rustful::{Server, Context, Response, TreeRouter};
use rustful::header::ContentType;

mod arguments;
mod config;
mod rest_methods;
mod executor;
mod session;
mod persistence;
mod captchatools;

use arguments::parse_arguments;
use config::Config;
use rest_methods::{req_create_captcha, req_get_catpcha, req_check_solution};
use captcha::{init, done};

// The code here is responsible for:
// * reading command line arguments (via module arguments.rs)
// * reading configuration file (via module config.rs)
// * running the server, receiving requests and sending responses (via rustful)

#[derive(Clone, Copy)]
enum RequestType {
    CreateCaptcha,
    GetCaptcha,
    CheckSolution,
}

struct Handler(Config, RequestType);

impl rustful::Handler for Handler {

    fn handle_request(&self, context: Context, mut response: Response) {

        let config = self.0.clone();
        let req_type = self.1;

        let (body, status) = match req_type {
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
}

fn main() {
    env_logger::init().unwrap();

    let conf = match parse_arguments() {
        Some(args) => {
            match Config::parse_config(&args.config_file) {
                Ok(conf) => conf,
                Err(msg) => {
                    error!("Could not read configuration file: {}", msg);
                    return;
                }
            }
        }
        _ => { return; }
    };

    info!(target: "main", "Starting server on port {} ...", conf.port);

    init();

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

    done();

    match srv {
        Ok(_)  => { info!("Quit.") },
        Err(e) => { error!("Could not start server: {}", e.description()) }
    }
}
