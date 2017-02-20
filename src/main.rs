#[macro_use] extern crate log;
extern crate env_logger;
extern crate getopts;
// see issue: https://github.com/rust-lang-nursery/rustc-serialize/issues/61
extern crate rustc_serialize;

extern crate redis;
#[macro_use] extern crate rustful;

use std::error::Error;
use std::io::Read;

use rustful::{Server, Context, Response, TreeRouter, StatusCode};
use rustful::header::{ContentType, Location};

mod arguments;
mod executor;
mod config;
mod persistence;
mod captcha;
mod image;
mod generator;
mod rest_methods;
mod session;

use arguments::parse_arguments;
use config::Config;
use rest_methods::{req_create_captcha, req_get_catpcha, req_check_solution};


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
        let rt = self.1;

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
}

fn main() {
    env_logger::init().unwrap();

    let args = match parse_arguments() {
        Some(a) => a,
        _ => { return; }
    };

    let conf = match Config::parse_config(&args.config_file) {
        Ok(a) => a,
        Err(msg) => {
            error!("Could not read configuration file: {}", msg);
            return;
        }
    };

    info!(target: "main", "Starting server on port {} ...", conf.port);

    image::init_img();

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

    image::done_img();

    match srv {
        Ok(_)  => { },
        Err(e) => { error!("Could not start server: {}", e.description()) }
    }
}
