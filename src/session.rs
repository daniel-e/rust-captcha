extern crate rand;

use self::rand::os::OsRng;
use self::rand::Rng;

static SESSION_CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyz";

#[derive(Clone)]
pub struct Session {
    id: String
}

impl Session {

    pub fn new() -> Option<Session> {
        // TODO: how expensive is it to create a new PRNG for every request?
        match OsRng::new() {
            Err(_) => None,
            Ok(mut rng) => {
                let c = SESSION_CHARS.chars().collect::<Vec<_>>();
                let mut s = String::new();
                for _ in 0..20 {
                    s.push(*rng.choose(&c).unwrap());
                }
                Some(Session { id: s })
            }
        }
    }

    pub fn from_string(s: String) -> Option<Session> {

        let b = s.chars().count() == 20 && s.chars().all(|x| SESSION_CHARS.contains(x));
        match b {
            true  => Some(Session { id: s} ),
            false => None
        }
    }

    pub fn to_string(&self) -> String {
        self.id.clone()
    }
}
