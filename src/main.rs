#[macro_use] extern crate rustful;
extern crate redis;

// see issue: https://github.com/rust-lang-nursery/rustc-serialize/issues/61
extern crate rustc_serialize;

#[macro_use] extern crate log;
extern crate env_logger;
extern crate getopts;

use std::error::Error;

use rustful::{Server, Context, Response, TreeRouter, Method};
use rustful::header::ContentType;
use redis::Client;

mod arguments;
mod executor;
mod config;
mod persistence;

use arguments::{Arguments, parse_arguments};
use executor::{new_captcha, JsonType};
use config::{parse_config, Config};

//let person = match context.variables.get("name") {
//  Some(name) => name,
//  None => "stranger".into()
//};

fn do_post(context: Context, mut response: Response, config: &Config) {
  match new_captcha(config) {
    Some(c) => {
      let mime_type = content_type!(Application / Json; Charset = Utf8);
      response.headers_mut().set(ContentType(mime_type));
      response.send(c.to_json(JsonType::Creation));
    },
    _ => { // Error TODO
    }
  }
}

fn do_get(context: Context, response: Response, config: &Config) {
  // TODO
  response.send("get");
}

fn do_request(context: Context, response: Response, config: &Config) {
  match context.method {
    Method::Post => do_post(context, response, config),
    Method::Get  => do_get(context, response, config),
    _ => ()
  }
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
        "/person/:name" => Get: Handler(do_request, conf.clone()),
      }
    },
    ..Server::default() // for the rest use default values
  }.run();

  match srv {
    Ok(_)  => { },
    Err(e) => { error!("Could not start server: {}", e.description()) }
  }
}
