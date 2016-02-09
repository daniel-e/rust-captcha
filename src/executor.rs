use super::config::Config;
use super::persistence::{persist, get, PersistenceError};
use super::captcha::{Captcha, CaptchaCreation, CaptchaToJson, CaptchaSolutionResponse, CaptchaSolution};
use super::captcha::CaptchaSolutionConstraints;
use super::session::Session;

pub enum ExecutorError {
    ConnectionFailed,
    NotFound,
    JsonError,
    ValidationError,
    NoRng,
    DatabaseError,
}

pub struct CaptchaResult {
    pub captcha: CaptchaCreation,
    pub session: String
}

// ----------------------------------------------------------------------------

pub fn check_captcha(session: Session, cs: CaptchaSolution, cf: Config) -> Result<CaptchaSolutionResponse, ExecutorError> {

    if !validate_solution(cs.solution.clone(), cf.min_letters, cf.max_letters, &cf.characters) {
        warn!(target: "check_captcha()", "Validation of solution failed.");
        return Err(ExecutorError::ValidationError);
    }

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
                match persist(&Captcha::new(&r, &session.to_string(), &c.solution), cf) {
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
        Ok(c)  => Ok(CaptchaCreation::new(c)),
        Err(e) => Err(map_error(e))
    }
}

/// Creates a new CAPTCHA and persists it in a database.
pub fn create_and_persist_captcha(conf: Config) -> Result<CaptchaResult, ExecutorError> {

    let session = Session::new().unwrap().to_string(); // TODO
    let solution = CaptchaSolution::new(CaptchaSolutionConstraints::new(&conf)).unwrap().to_string();

    // TODO create the image

    let c = Captcha {
        tries: 0,
        max_tries: conf.max_tries,
        solved: false,
        session: session,
        solution: solution,
    };

    info!(target: "executor::create_and_persist_captcha()", "Created new CAPTCHA: {}", c.to_json());

    match persist(&c, conf) {
        Ok(_) => {
            let session = c.session.clone();
            Ok(CaptchaResult {
                captcha: CaptchaCreation::new(c),
                session: session,
            })
        }
        Err(e) => Err(map_error(e))
    }
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

fn validate_solution(s: String, minlen: usize, maxlen: usize, chars: &String) -> bool {

    let l = s.chars().count();
    l >= minlen && l <= maxlen && s.chars().all(|x| chars.contains(x))
}
