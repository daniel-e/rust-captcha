use super::redis::{Client, Commands, Connection, RedisResult};

use captchatools::{Captcha, CaptchaToJson};
use config::Config;
use session::Session;

pub enum PersistenceError {
    ConnectionFailed,
    KeyNotFound,
    JsonError,
    DatabaseError,
}

// ----------------------------------------------------------------------------

pub fn get(session: Session, conf: Config) -> Result<Captcha, PersistenceError> {

    match get_connection(conf.redis_ip) {
        Ok(con) => get_key(con, session.to_string()),
        Err(e)  => {
            info!(target: "persistence::get()", "Could not retrieve session [{}]", session.to_string());
            Err(e)
        }
    }
}

// PersistenceError in [DatabaseError]
pub fn persist(c: &Captcha, conf: Config) -> Result<(), PersistenceError> {

    let k = c.session.clone();

    match get_connection(conf.redis_ip) {
        Ok(con) => {
            debug!(target: "persist", "persisting {}", c.to_json());
            set_key(con, k, c.to_json(), conf.expire)
        },
        _ => {
            info!(target: "persistence::persist()", "Could not persist [{}]", k);
            Err(PersistenceError::DatabaseError)
        }
    }
}

// ----------------------------------------------------------------------------

fn set_key(con: Connection, k: String, v: String, seconds: usize) -> Result<(), PersistenceError> {

    let r: RedisResult<String> = con.set_ex(k.clone(), v, seconds);
    match r {
        Err(_) => {
            error!(target: "persistence::set_key()", "Could not store value for [{}]", k);
            Err(PersistenceError::DatabaseError)
        },
        _ => Ok(())
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

fn get_connection(ip: String) -> Result<Connection, PersistenceError> {

    let addr = "redis://".to_string() + &ip + "/";
    match Client::open(&addr[..]) {
        Ok(client) => connect(client),
        Err(_) => {
            error!(target: "persistence::get_connection()", "Could not connect to Redis.");
            Err(PersistenceError::ConnectionFailed)
        }
    }
}
