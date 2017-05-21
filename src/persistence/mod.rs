mod error;
mod item;

use std::env;
use time;
use redis::{Client, Commands, RedisResult, Connection};
use serde_json;

// exports
pub use self::error::Error;
pub use self::item::{build_item, Item};

pub type QueryResult = Result<Item, Error>;

pub struct Persistence { }

impl Persistence {
    pub fn set(i: Item) -> Result<(), Error> {
        connect()?
            .set_ex::<String, String, String>(key(i.uuid()), serde_json::to_string(&i).map_err(|_| Error::Json)?, ttl(&i))
            .map_err(|_| Error::Connection)
            .map(|_| ())
    }

    pub fn get<T: ToString>(uuid: T) -> QueryResult {
        Ok(parse_result(connect()?.get(key(uuid.to_string())))?)
    }

    pub fn del<T: ToString>(uuid: T) {
        connect().ok().and_then(|c| c.del::<String, Option<String>>(key(uuid.to_string())).ok());
    }
}

// -------------------------------------------------------------------------------------------------

pub fn ttl(i: &Item) -> usize {
    let d = i.expires() - time::now().to_timespec().sec;
    if d > 0 {
        d as usize
    } else {
        1
    }
}

fn key(k: String) -> String {
    format!("X1:{}", k)
}

fn connect() -> Result<Connection, Error> {
    Ok(Client::open(address()?.as_str())
        .map_err(|_| Error::Connection)?
        .get_connection().map_err(|_| Error::Connection)?
    )
}

fn address() -> Result<String, Error> {
    Ok(format!("redis://{}/", env::var("REDIS_HOST").map_err(|_| Error::NoLocation)?))
}

fn parse_string(val: String) -> QueryResult {
    let d: Item = serde_json::from_str(&val).map_err(|_| Error::Json)?;
    Ok(d)
}

fn parse_option(o: Option<String>) -> QueryResult {
    o.ok_or(Error::NotFound).and_then(parse_string)
}

fn parse_result(r: RedisResult<Option<String>>) -> QueryResult {
    r.map_err(|_| Error::Connection).and_then(parse_option)
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::env;
    use persistence::{Error, address, Persistence, parse_result, build_item};
    use std::thread::sleep;
    use std::time::Duration;
    use std::io::{self, ErrorKind};
    use redis::RedisError;
    use time;

    // For the following tests Redis must be running.

    fn as_some(s: &str) -> Option<String> {
        Some(s.to_string())
    }

    #[test]
    fn test_parse_result() {
        assert_eq!(
            parse_result(Err(RedisError::from(io::Error::new(ErrorKind::Other, "x")))).err().unwrap(), Error::Connection
        );

        assert_eq!(parse_result(Ok(None)).err().unwrap(), Error::NotFound);

        let s = "{\"uuid\":\"x\",\"solution\":\"solution\",\"tries_left\":3,\"expires\":12345678}";
        let i = build_item()
            .uuid("x")
            .solution("solution")
            .tries_left(3)
            .expires(time::at(time::Timespec{ sec: 12345678, nsec: 0}))
            .item()
            .expect("build item");
        assert_eq!(parse_result(Ok(as_some(s))), Ok(i));

        assert_eq!(parse_result(Ok(as_some("a"))).err().unwrap(), Error::Json);
    }

    #[test]
    fn test_address() {
        env::remove_var("REDIS_HOST");
        assert_eq!(address().err().unwrap(), Error::NoLocation);

        env::set_var("REDIS_HOST", "localhost");
        assert_eq!(address().unwrap(), "redis://localhost/");

        env::remove_var("REDIS_HOST");
    }

    #[test]
    fn test_notfound() {
        env::set_var("REDIS_HOST", "localhost");

        // Search an element that does not exist.
        assert_eq!(Persistence::get("xx").expect_err("a"), Error::NotFound);

        env::remove_var("REDIS_HOST");
    }

    #[test]
    fn test_expire() {
        env::set_var("REDIS_HOST", "localhost");

        // Insert an element that will be expired after 1 second.
        let i = build_item()
            .uuid("uid1234")
            .solution("sol1234")
            .tries_left(3)
            .ttl(1)
            .item().expect("building item");
        assert!(Persistence::set(i).is_ok());

        // Check that the element exists.
        assert_eq!(Persistence::get("uid1234").expect("b").solution(), "sol1234");

        // Wait that the element is removed from Redis ...
        sleep(Duration::from_secs(2));

        // Check that item is removed.
        assert_eq!(Persistence::get("uid1234").expect_err("c"), Error::NotFound);

        env::remove_var("REDIS_HOST");
    }

    #[test]
    fn test_persist() {
        env::set_var("REDIS_HOST", "localhost");

        // Insert an element that will expire after 1 second.
        let i = build_item()
            .uuid("uid_persist")
            .solution("solp")
            .tries_left(3)
            .ttl(1)
            .item()
            .expect("building item");
        assert!(Persistence::set(i).is_ok());

        // Check that the element exists.
        assert_eq!(Persistence::get("uid_persist").expect("b").solution(), "solp");

        env::remove_var("REDIS_HOST");
    }

    #[test]
    fn test_delete() {
        env::set_var("REDIS_HOST", "localhost");

        // Insert an element that will be expired after 10 second.
        let i = build_item()
            .uuid("uidr")
            .solution("solution123")
            .tries_left(3)
            .ttl(10)
            .item()
            .expect("building item");
        assert!(Persistence::set(i).is_ok());

        // Check that the element does exist.
        assert_eq!(Persistence::get("uidr").unwrap().solution(), "solution123");

        // Remove that item
        Persistence::del("uidr");

        // Check that item is removed.
        assert_eq!(Persistence::get("uidr").expect_err("e"), Error::NotFound);

        env::remove_var("REDIS_HOST");
    }
}

