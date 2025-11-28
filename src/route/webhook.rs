use crate::route::base::Route;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

pub struct WebhookRoute {
    client: Client,
    url: String,
}

impl WebhookRoute {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }
}

#[async_trait]
impl Route for WebhookRoute {
    async fn send(&self, update: Value) {
        let _ = self.client.post(&self.url).json(&update).send().await;
    }
}