// see issue: https://github.com/rust-lang-nursery/rustc-serialize/issues/61
extern crate rustc_serialize;
extern crate redis;

mod executor;
mod arguments;
mod config;
mod persistence;
