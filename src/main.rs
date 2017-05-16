#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;

use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

const PORT: u32 = 8080;
const THREADS: usize = 8;

fn incoming(req: Request, mut res: Response) {
    *res.status_mut() = StatusCode::BadRequest;
    res.headers_mut().set(ContentType(
            Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)]))
    );
    res.send(b"hello");
}

fn main() {
    env_logger::init().expect("initializing logger failed");

    info!(target: "main", "Starting server on port {} ...", PORT);

    Server::http(format!("0.0.0.0:{}", PORT))
        .expect("http failed")
        .handle_threads(incoming, THREADS)
        .expect("handle failed");
}
