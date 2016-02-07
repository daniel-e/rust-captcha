extern crate rand;

use super::rustc_serialize::json;
use self::rand::os::OsRng;
use self::rand::Rng;

use super::config::Config;
use super::persistence::persist;

#[derive(RustcDecodable, RustcEncodable)]
struct JsonCreation {
  solved: bool,
  tries: u32,
  max_tries: u32,
}

pub enum JsonType {
  All,
  Creation
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Captcha {
  pub solution: String,
  pub tries: u32,
  pub max_tries: u32,
  pub session: String,
  pub solved: bool,
}

impl Captcha {

  pub fn to_json(&self, jt: JsonType) -> String {
    match jt {
      JsonType::Creation => {
        let js = JsonCreation {
          solved: self.solved,
          tries: self.tries,
          max_tries: self.max_tries
        };
        json::encode(&js).unwrap() // TODO
      },
      JsonType::All => {
        json::encode(&self).unwrap() // TODO
      }
    }
  }
}

pub fn new_captcha(conf: &Config) -> Option<Captcha> {
  let c = Captcha {
    tries: 0,
    max_tries: conf.max_tries,
    solved: false,
    session: new_session(),
    solution: new_solution(conf.min_letters, conf.max_letters, &conf.characters),
  };

  //info!(target: "executor", "Created new CAPTCHA: {}", c.to_json(JsonType::All));
  println!("Created new CAPTCHA: {}", c.to_json(JsonType::All));

  if !persist(& c) {
    return None;
  }
  Some(c)
}

fn new_session() -> String {

  // TODO: how expensive is it to create a new PRNG for every request?
  let mut rng = OsRng::new().unwrap(); // TODO
  let mut s = rng.gen_ascii_chars().take(20).collect::<_>();
  s
}

fn new_solution(minlen: u32, maxlen: u32, chars: &String) -> String {

  let mut rng = OsRng::new().unwrap(); // TODO
  let l = rng.next_u32() % (maxlen - minlen + 1) + minlen;
  let mut s = String::new();
  let c: Vec<char> = chars.chars().collect::<_>();

  for _ in 0..l {
    s.push(*rng.choose(&c).unwrap());
  }
  s
}
