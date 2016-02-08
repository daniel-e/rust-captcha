use super::redis::{Client, Commands, Connection, RedisResult};

use captcha::{Captcha, CaptchaToJson};
use super::config::Config;

pub enum PersistenceError {
    ConnectionFailed,
    KeyNotFound,
    JsonError,
}

// ----------------------------------------------------------------------------

pub fn get(session: String, conf: &Config) -> Result<Captcha, PersistenceError> {

    match get_connection(&conf.redis_ip) {
        Ok(con) => get_key(con, session),
        Err(e)  => {
            info!(target: "persistence::get()", "Could not retrieve session [{}]", session);
            Err(e)
        }
    }
}

pub fn persist(c: &Captcha, conf: &Config) -> bool {

    let k = c.session.clone();

    match get_connection(&conf.redis_ip) {
        Ok(con) => {
            set_key(con, k, c.to_json(), conf.expire)
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

fn get_connection(ip: &String) -> Result<Connection, PersistenceError> {

    let addr = "redis://".to_string() + ip + "/";
    match Client::open(&addr[..]) {
        Ok(client) => connect(client),
        Err(_) => {
            error!(target: "persistence::get_connection()", "Could not connect to Redis.");
            Err(PersistenceError::ConnectionFailed)
        }
    }
}
