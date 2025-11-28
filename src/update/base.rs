use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait UpdateProvider: Send + Sync {
    async fn start(&self, tx: Sender<Value>);
}