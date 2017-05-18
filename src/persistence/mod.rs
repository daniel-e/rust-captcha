use std::env;
use redis::{Client, Commands, RedisResult};

pub type QueryResult = Result<(String, u32), PersistenceError>;

#[derive(Debug, PartialEq)]
pub enum PersistenceError {
    NotFound,
    NoLocation,
    Connection,
    InvalidData
}

pub fn persist(uuid: String, solution: String, max_tries: u32, ttl: u32) -> Result<(), PersistenceError> {

    let c = Client::open(address()?.as_str()).map_err(|_| PersistenceError::Connection)?;
    let con = c.get_connection().map_err(|_| PersistenceError::Connection)?;

    let data = solution + ":" + max_tries.to_string().as_str();
    let r: RedisResult<String> = con.set_ex(uuid, data, ttl as usize);
    r.map_err(|_| PersistenceError::Connection).map(|_| ())
}

pub fn from_persistence(uuid: String) -> QueryResult {

    let c = Client::open(address()?.as_str()).map_err(|_| PersistenceError::Connection)?;
    let con = c.get_connection().map_err(|_| PersistenceError::Connection)?;

    Ok(parse_result(con.get(uuid))?)
}

// -------------------------------------------------------------------------------------------------

fn address() -> Result<String, PersistenceError> {
    let l = env::var("REDIS_HOST").map_err(|_| PersistenceError::NoLocation)?;
    Ok("redis://".to_string() + l.as_str() + "/")
}

fn parse_string(val: String) -> QueryResult {
    let arr = val.split(":").collect::<Vec<_>>();
    match arr.len() {
        2 => Ok((arr[0].to_string(), arr[1].parse::<u32>().map_err(|_| PersistenceError::InvalidData)?)),
        _ => Err(PersistenceError::InvalidData)
    }
}

fn parse_option(o: Option<String>) -> QueryResult {
    o.ok_or(PersistenceError::NotFound).and_then(parse_string)
}

fn parse_result(r: RedisResult<Option<String>>) -> QueryResult {
    r.map_err(|_| PersistenceError::Connection).and_then(parse_option)
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::env;
    use persistence::{PersistenceError, address, persist, from_persistence};
    use std::thread::sleep;
    use std::time::Duration;

    // For the following tests Redis must be running.

    #[test]
    fn test_address() {
        env::remove_var("REDIS_HOST");
        assert_eq!(address().err().unwrap(), PersistenceError::NoLocation);

        env::set_var("REDIS_HOST", "localhost");
        assert_eq!(address().unwrap(), "redis://localhost/");

        env::remove_var("REDIS_HOST");
    }

    #[test]
    fn test_persist() {
        env::set_var("REDIS_HOST", "localhost");

        // Search an element that does not exist.
        assert_eq!(from_persistence("xx".to_string()).err().unwrap(), PersistenceError::NotFound);

        // Insert an element that is expired after 1 second.
        assert!(persist("uid".to_string(), "solution".to_string(), 3, 1).is_ok());

        // Get an element that does exist.
        assert_eq!(from_persistence("uid".to_string()).unwrap(), ("solution".to_string(), 3));

        // Wait that the element is removed from Redis ...
        sleep(Duration::from_secs(2));

        // Check that item is removed.
        assert_eq!(from_persistence("uid".to_string()).err().unwrap(), PersistenceError::NotFound);

        env::remove_var("REDIS_HOST");
    }
}

