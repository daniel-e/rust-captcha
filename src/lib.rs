#[macro_use]
extern crate log;
extern crate rustful;
extern crate captcha;
extern crate uuid;
extern crate base64;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate redis;
extern crate hyper;
extern crate time;

pub mod methods;
pub mod requesthandler;
pub mod validation;
pub mod persistence;