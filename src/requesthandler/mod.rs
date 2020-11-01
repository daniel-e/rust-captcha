use methods::{CaptchaError, captcha_new, captcha_solution, captcha_newget};

fn client_id() -> String {
    String::from("<unknown>")
    // TODO ROCKET
    /*
    String::from_utf8(
        c.headers.get_raw("x-client-id").unwrap_or(&[]).iter().next().unwrap_or(&u.clone().into_bytes()).clone()
    ).ok()
     .unwrap_or(u)
     .chars()
     .filter(|c| !c.is_control())
     .collect()*/
}

pub fn req_captcha_newget(difficulty: String) -> Result<String, CaptchaError> {
    let clientid = client_id(); // TODO ROCKET
    match captcha_newget(difficulty) {
        Ok(details) => {
            info!("Created new CAPTCHA [{}], clientid [{}].", details.uuid(), clientid);
            Ok(details.as_json())
        },
        Err(e)      => {
            match e {
                CaptchaError::NotFound | CaptchaError::InvalidParameters => info!("Failed to create new CAPTCHA [{:?}], clientid [{}].", e, clientid),
                _ => error!("Failed to create new CAPTCHA [{:?}], clientid [{}].", e, clientid)
            }
            Err(e)
        }
    }
}

pub fn req_captcha_new(difficulty: String, max_tries: String, ttl: String) -> Result<String, CaptchaError> {
    let clientid = client_id(); // TODO ROCKET
    match captcha_new(difficulty, max_tries, ttl) {
        Ok(details) => {
            info!("Created new CAPTCHA [{}], clientid [{}].", details.uuid(), clientid);
            Ok(details.as_json())
        },
        Err(e)      => {
            match e {
                CaptchaError::NotFound | CaptchaError::InvalidParameters => info!("Failed to create new CAPTCHA [{:?}], clientid [{}].", e, clientid),
                _ => error!("Failed to create new CAPTCHA [{:?}], clientid [{}].", e, clientid)
            }
            Err(e)
        }
    }
}

pub fn req_captcha_solution(id: String, solution: String) -> Result<String, CaptchaError> {
    let clientid = client_id(); // TODO ROCKET
    match captcha_solution(id, solution) {
        Ok(details) => {
            info!("Solution checked for [{}] [{}] [{}], clientid [{}].", details.uuid(), details.csr().result(), details.csr().reason(), clientid);
            Ok(details.as_json())
        },
        Err(e)      => {
            match e {
                CaptchaError::NotFound | CaptchaError::InvalidParameters => info!("Failed to check solution [{:?}], clientid [{}].", e, clientid),
                _ => error!("Failed to check solution [{:?}], clientid [{}].", e, clientid)
            }
            Err(e)
        }
    }
}
