use methods::{CaptchaError, captcha_new, captcha_solution, captcha_newget};

pub fn req_captcha_newget(difficulty: String, clientid: String) -> Result<String, CaptchaError> {
    match captcha_newget(difficulty) {
        Ok(details) => {
            info!("Created new CAPTCHA [{}], clientid [{}].", details.uuid(), clientid);
            Ok(details.as_json())
        },
        Err(e) => {
            match e {
                CaptchaError::NotFound | CaptchaError::InvalidParameters => info!("Failed to create new CAPTCHA [{:?}], clientid [{}].", e, clientid),
                _ => error!("Failed to create new CAPTCHA [{:?}], clientid [{}].", e, clientid)
            }
            Err(e)
        }
    }
}

pub fn req_captcha_new(difficulty: String, max_tries: String, ttl: String, clientid: String) -> Result<String, CaptchaError> {
    match captcha_new(difficulty, max_tries, ttl) {
        Ok(details) => {
            info!("Created new CAPTCHA [{}], clientid [{}].", details.uuid(), clientid);
            Ok(details.as_json())
        },
        Err(e) => {
            match e {
                CaptchaError::NotFound | CaptchaError::InvalidParameters => info!("Failed to create new CAPTCHA [{:?}], clientid [{}].", e, clientid),
                _ => error!("Failed to create new CAPTCHA [{:?}], clientid [{}].", e, clientid)
            }
            Err(e)
        }
    }
}

pub fn req_captcha_solution(id: String, solution: String, clientid: String) -> Result<String, CaptchaError> {
    match captcha_solution(id, solution) {
        Ok(details) => {
            info!("Solution checked for [{}] [{}], clientid [{}].", details.uuid(), details.csr().result(), clientid);
            Ok(details.as_json())
        },
        Err(e) => {
            match e {
                CaptchaError::NotFound | CaptchaError::InvalidParameters => info!("Failed to check solution [{:?}], clientid [{}].", e, clientid),
                _ => error!("Failed to check solution [{:?}], clientid [{}].", e, clientid)
            }
            Err(e)
        }
    }
}
