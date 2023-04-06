extern crate redis;

pub fn check_redis(message_id: &str) -> redis::RedisResult<String> {
    use redis::Commands;
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let found = con.get(&message_id);
    match found {
        Ok(_) => println!("Found {} in Redis.", &message_id),
        Err(_) => println!("Not found in Redis:"),
    }
    found
}

pub fn set_redis(message_id: &str) {
    use redis::Commands;
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection();
    match con {
        Ok(mut conn) => {
            let _: () = conn.set(message_id, true).unwrap();
        }
        Err(_) => (),
    };
}
