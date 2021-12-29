use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};

use crate::app::entity;
use redis::Commands;

pub struct Paste {
    connection: Arc<Mutex<redis::Connection>>,
}

impl Paste {
    pub fn new(conn: Arc<Mutex<redis::Connection>>) -> Self {
        Self { connection: conn }
    }
    pub fn query(&mut self, key: &str) -> Result<entity::Paste, redis::RedisError> {
        let mut connection = self.connection.borrow_mut().lock().unwrap();
        let result = connection.get(key)?;
        Ok(result)
    }
    pub fn create(
        &mut self,
        key: &str,
        value: &entity::Paste,
        expire_seconds: Option<usize>,
    ) -> Result<(), redis::RedisError> {
        let mut connection = self.connection.borrow_mut().lock().unwrap();
        if let Some(expire_seconds) = expire_seconds {
            connection.set_ex(key, value, expire_seconds)?;
        } else {
            connection.set(key, value)?;
        }
        Ok(())
    }
}
