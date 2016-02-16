use super::rustc_serialize::base64::ToBase64;
use super::rustc_serialize::base64;

use config::Config;
use persistence::{persist, get, PersistenceError};
use captcha::{Captcha, CaptchaCreation, CaptchaToJson, CaptchaSolutionResponse, CaptchaSolution};
use captcha::CaptchaSolutionConstraints;
use session::Session;
use generator::{CharConfig, captcha_png};

pub enum ExecutorError {
    ConnectionFailed,
    NotFound,
    JsonError,
    NoRng,
    DatabaseError,
    GeneratorFailed,
}

pub struct CaptchaResult {
    pub captcha: CaptchaCreation,
    pub session: String
}

// ----------------------------------------------------------------------------

pub fn check_captcha(session: Session, cs: CaptchaSolution, cf: Config) -> Result<CaptchaSolutionResponse, ExecutorError> {

    match get(session.clone(), cf.clone()) { // get catpcha from Redis
        Ok(c) => {
            let cr = CaptchaSolutionResponse::new(&c);
            if cr.tries >= cr.max_tries {
                Ok(cr.set_reason("Too many tries.").set_checked(false))
            } else if cr.solved {
                Ok(cr.set_reason("Already solved.").set_checked(false))
            } else {
                let r =
                    if c.solution == cs.solution {
                        cr.inc_tries().set_checked(true).set_reason("Correct.").set_solved(true)
                    } else {
                        cr.inc_tries().set_checked(true).set_reason("Incorrect.")
                    };
                match persist(&Captcha::new(&r, &session.to_string(), &c.solution, c.png_data), cf) {
                    Ok(_) => Ok(r),
                    Err(e) => Err(map_error(e))
                }
            }
        },
        Err(e) => Err(map_error(e))
    }
}

pub fn get_captcha(session: Session, conf: Config) -> Result<CaptchaCreation, ExecutorError> {

    match get(session, conf) {
        Ok(c) => Ok(CaptchaCreation::new(c)),
        Err(e) => Err(map_error(e))
    }
}

fn generate_image(solution: &str) -> Result<String, ExecutorError> {

    let cc = CharConfig { // TODO read from config
        min_angle: -10.0,
        max_angle: 10.0,
        min_size: 56,
        max_size: 72,
        colors: vec!["black".to_string()],
        fonts: vec!["Verdana-Bold-Italic".to_string()]
    };

    match captcha_png(solution, &cc) {
        Ok(png) => {
            let bc = base64::Config {
                char_set: base64::CharacterSet::Standard,
                newline: base64::Newline::LF,
                pad: true,
                line_length: None
            };
            Ok(png[..].to_base64(bc))
        },
        Err(_) => {
            Err(ExecutorError::GeneratorFailed)
        }
    }
}

/// Creates a new CAPTCHA and persists it in a database.
pub fn create_and_persist_captcha(conf: Config) -> Result<CaptchaResult, ExecutorError> {

    Session::new().map_or(Err(ExecutorError::NoRng), |session| {
        let c = CaptchaSolutionConstraints::new(&conf);
        CaptchaSolution::new(c).map_or(Err(ExecutorError::NoRng), |solution| {

            let b64 = try!(generate_image(&solution.to_string()));

            let captcha = Captcha {
                png_data: b64,
                tries: 0,
                max_tries: conf.max_tries,
                solved: false,
                session: session.to_string(),
                solution: solution.to_string(),
            };

            info!(target: "create_and_persist_captcha()", "Created new CAPTCHA: {}", captcha.to_json());

            match persist(&captcha, conf) {
                Ok(_) => {
                    Ok(CaptchaResult {
                        captcha: CaptchaCreation::new(captcha),
                        session: session.to_string(),
                    })
                }
                Err(e) => Err(map_error(e))
            }
        })
    })
}

// ----------------------------------------------------------------------------

fn map_error(e: PersistenceError) -> ExecutorError {
    match e {
        PersistenceError::ConnectionFailed => ExecutorError::ConnectionFailed,
        PersistenceError::KeyNotFound      => ExecutorError::NotFound,
        PersistenceError::JsonError        => ExecutorError::JsonError,
        PersistenceError::DatabaseError    => ExecutorError::DatabaseError,
    }
}
