use captcha::{Captcha, CaptchaToJson};

use super::redis::{Client, Commands, Connection, RedisResult};

pub fn persist(c: &Captcha) -> bool {

    let seconds = 120;  // TODO config
    let k = c.session.clone();
    let v = c.to_json();

    let con = match get_connection() {
        Some(c) => { c },
        _ => {
            info!(target: "persistence::persist()", "Could not persist [{}]", k);
            return false;
        }
    };

    let r: RedisResult<String> = con.set_ex(k.clone(), v, seconds);
    match r {
        Err(_) => {
            error!(target: "persistence::persist()", "Could not store value for [{}]", k);
            false
        },
        _ => true
    }
}

// ----------------------------------------------------------------------------

fn get_connection() -> Option<Connection> {

    // TODO configuration
    let client = match Client::open("redis://127.0.0.1/") {
        Ok(c) => { c }
        _ => {
            error!(target: "persistence::get_connection()", "Could not connect to Redis.");
            return None;
        }
    };

    match client.get_connection() {
        Ok(con) => {
            Some(con)
        },
        _ => {
            error!(target: "persistence::get_connection()", "Could not get connection to Redis.");
            None
        }
    }
}
