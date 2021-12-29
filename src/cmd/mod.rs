use crate::app;
use redis;

pub fn boost() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    app::run("127.0.0.1:8080", client)?;
    Ok(())
}
