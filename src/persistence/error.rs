#[derive(Debug, PartialEq)]
pub enum Error {
    NotFound,
    NoLocation,
    Connection,
    Json,
}