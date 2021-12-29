use redis;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub connection: Arc<Mutex<redis::Connection>>,
}
