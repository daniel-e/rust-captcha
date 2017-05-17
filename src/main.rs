#[macro_use]
extern crate log;
#[macro_use]
extern crate rustful;
extern crate env_logger;
extern crate rust_captcha;

use rustful::{Server, TreeRouter};
use std::error::Error;

use rust_captcha::requesthandler::{RequestHandler, CaptchaMethod};

const PORT: u16 = 8080;

fn main() {
    env_logger::init().expect("initializing logger failed");

    info!(target: "main", "Starting server on port {} ...", PORT);

    let ret = Server {
        handlers: insert_routes! {
            TreeRouter::new() => {
                "/new/:difficulty/:max_tries/:ttl" => Post: RequestHandler::new(CaptchaMethod::New),
                "/solution/:id/:solution"          => Post: RequestHandler::new(CaptchaMethod::Solution)
            }
        },
        host: PORT.into(),
        ..Server::default()
    }.run();

    match ret {
        Ok(_)  => { },
        Err(e) => println!("could not start server: {}", e.description())
    }
}
