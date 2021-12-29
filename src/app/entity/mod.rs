use chrono::Utc;
use redis::{from_redis_value, ErrorKind, FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use serde_json;

pub trait Entity
where
    Self: Sized + FromRedisValue + ToRedisArgs + ToString,
{
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paste {
    pub lang: String,
    pub plain: String,
    pub html: String,
    pub poster: String,
    pub time: chrono::DateTime<Utc>,
}

impl FromRedisValue for Paste {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let v: String = from_redis_value(v)?;
        Ok(serde_json::from_str::<Self>(&v).or(Err((ErrorKind::TypeError, "serde error")))?)
    }
}

impl ToRedisArgs for Paste {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(serde_json::to_string(self).unwrap().as_bytes())
    }
}

impl ToString for Paste {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Entity for Paste {}
