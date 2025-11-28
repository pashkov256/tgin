use crate::route::base::Route;
use async_trait::async_trait;
use serde_json::Value;

pub struct LongPollRoute;

impl LongPollRoute {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Route for LongPollRoute {
    async fn send(&self, _update: Value) {
        // Логика складывания апдейтов в очередь для последующего getUpdates от бота
    }
}