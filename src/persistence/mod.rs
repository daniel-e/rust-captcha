use redis::{Client, Commands, RedisResult};

pub fn persist(uuid: String, solution: String, max_tries: u32, ttl: u32) -> Result<(), ()> {

    // TODO replace localhost
    let addr = "redis://localhost/";
    let client = Client::open(addr).map_err(|_| ())?;
    let con = client.get_connection().map_err(|_| ())?;

    let data = solution + ":" + max_tries.to_string().as_str();
    let r: RedisResult<String> = con.set_ex(uuid, data, ttl as usize);
    r.map_err(|_| ()).map(|_| ())
}