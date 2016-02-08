use captcha::{Captcha, CaptchaToJson};

use super::redis::{Client, Commands, Connection, RedisResult};

pub enum PersistenceError {
    ConnectionFailed, // 503 ServiceUnavailable
    KeyNotFound,      // 404 NotFound
    JsonError         // 500 InternalServerError
}

// ----------------------------------------------------------------------------

pub fn get(session: String) -> Result<Captcha, PersistenceError> {

    match get_connection() {
        Ok(con) => {
            get_key(con, session)
        },
        Err(_) => {
            info!(target: "persistence::get()", "Could not retrieve session [{}]", session.clone());
            Err(PersistenceError::ConnectionFailed)
        }
    }
}

pub fn persist(c: &Captcha) -> bool {

    let seconds = 120;  // TODO config
    let k = c.session.clone();

    match get_connection() {
        Ok(con) => {
            set_key(con, k, c.to_json(), seconds)
        },
        _ => {
            info!(target: "persistence::persist()", "Could not persist [{}]", k);
            false
        }
    }
}

// ----------------------------------------------------------------------------

fn set_key(con: Connection, k: String, v: String, seconds: usize) -> bool {

    let r: RedisResult<String> = con.set_ex(k.clone(), v, seconds);
    match r {
        Err(_) => {
            error!(target: "persistence::set_key()", "Could not store value for [{}]", k);
            false
        },
        _ => true
    }
}

fn decode_json(json: String) -> Result<Captcha, PersistenceError> {

    match Captcha::from_json(json.clone()) {
        Some(c) => Ok(c),
        None => {
            error!(target: "persistence::decode_json()", "JSON decoder error [{}]", json);
            Err(PersistenceError::JsonError)
        }
    }
}

fn get_key(con: Connection, session: String) -> Result<Captcha, PersistenceError> {

    let r: RedisResult<String> = con.get(session.clone());
    match r {
        Ok(s) => {
            decode_json(s)
        },
        Err(_) => {
            error!(target: "persistence::get()", "Session not found [{}]", session);
            Err(PersistenceError::KeyNotFound)
        }
    }
}

fn connect(client: Client) -> Result<Connection, PersistenceError> {

    match client.get_connection() {
        Ok(con) => Ok(con),
        Err(_)  => {
            error!(target: "persistence::connect()", "Could not get connection to Redis.");
            Err(PersistenceError::ConnectionFailed)
        }
    }
}

fn get_connection() -> Result<Connection, PersistenceError> {

    // TODO configuration
    match Client::open("redis://127.0.0.1/") {
        Ok(client) => {
            connect(client)
        },
        Err(_) => {
            error!(target: "persistence::get_connection()", "Could not connect to Redis.");
            Err(PersistenceError::ConnectionFailed)
        }
    }
}
