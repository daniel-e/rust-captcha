extern crate rustful;
extern crate captcha;
extern crate uuid;
extern crate base64;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate redis;

pub mod methods;
pub mod requesthandler;
pub mod validation;
pub mod persistence;