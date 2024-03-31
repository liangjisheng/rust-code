use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

fn fetch_an_string() -> redis::RedisResult<String> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("name", "alice")?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("name")
}

fn main() {
    let v = match fetch_an_integer() {
        Ok(v) => v,
        Err(_err) => 0,
    };
    println!("res {}", v);

    let v = match fetch_an_string() {
        Ok(v) => v,
        Err(_err) => String::new(),
    };
    println!("res {}", v);
}
