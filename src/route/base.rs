use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Route: Send + Sync {
    async fn send(&self, update: Value);
}