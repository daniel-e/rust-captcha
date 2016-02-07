use executor::Captcha;
use executor::JsonType;

use super::redis::Client;
use super::redis::Commands;

// TODO check if key does already exist?
// TODO expire

pub fn persist(c: &Captcha) -> bool {

  // TODO config
  match Client::open("redis://127.0.0.1/") {
    Err(_) => {
      false
    },
    Ok(client) => {
      match client.get_connection() {
        Err(_) => {
          false
        },
        Ok(con) => {
          con.set(c.session.clone(), c.to_json(JsonType::All)).unwrap() // TODO
        }
      }
    }
  }
}
