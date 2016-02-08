extern crate getopts;

use self::getopts::Options;
use std::env;

pub struct Arguments {
  pub config_file: String,
}

pub fn parse_arguments() -> Option<Arguments> {

  let args: Vec<String> = env::args().collect();
  let prog = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("c", "config", "Configuration file.", "NAME");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m },
    Err(_) => {
      print_usage(prog, opts);
      return None
    }
  };

  let config = match matches.opt_str("c") {
    Some(s) => s,
    _ => {
      print_usage(prog, opts);
      return None
    }
  };

  Some(Arguments {
    config_file: config,
  })
}

fn print_usage(prog: String, opts: Options) {

  let brief = format!("Usage: {} FILE [options]", prog);
  print!("{}", opts.usage(&brief));
}
