use super::rustc_serialize::json;
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Config {
  pub max_tries: u32,
  pub min_letters: u32,
  pub max_letters: u32,
  pub characters: String,
}

pub fn parse_config(fname: &String) -> Result<Config, String> {

  let mut f = match File::open(fname) {
    Ok(f) => { f },
    Err(e) => {
      return Err(format!("Could not open file {} {}", fname, e.description())) 
    }
  };

  let mut data = String::new();
  match f.read_to_string(&mut data) {
    Err(e) => {
      return Err(format!("Could not read file: {}", e.description()))
    }
    _ => { }
  }

  let decoded: Config = match json::decode(&data) {
    Err(e) => {
      return Err(format!("Could not parse JSON: {}", e.description()))
    }
    Ok(c) => { c }
  };

  if decoded.min_letters > decoded.max_letters {
    return Err(
      "The value for min_letters must not be larger than max_letters.".to_string())
  }

  if decoded.min_letters == 0 {
    return Err(
      "The value for min_letters must greater than zero.".to_string())
  }

  if decoded.characters.is_empty() {
  return Err(
    "The value for characters must not be empty.".to_string())
  }

  Ok(decoded)
}
