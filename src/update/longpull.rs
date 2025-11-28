use crate::update::base::UpdateProvider;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use tokio::time::{sleep, Duration};

pub struct LongPollUpdate {
    token: String,
    client: Client,
    base_url: &'a str
}

impl LongPollUpdate {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::new(),
            base_url: &format!("https://api.telegram.org/bot{}/getUpdates", token)
        }
    }
}

#[async_trait]
impl UpdateProvider for LongPollUpdate {
    async fn start(&self, tx: Sender<Value>) {
        let mut offset = 0;

        loop {
            let params = [("offset", offset.to_string()), ("timeout", "30".to_string())];
            if let Ok(res) = self.client.get(self.base_url).query(&params).send().await {
                if let Ok(json) = res.json::<Value>().await {
                    if let Some(result) = json.get("result").and_then(|r| r.as_array()) {
                        for update in result {
                            if let Some(id) = update.get("update_id").and_then(|i| i.as_i64()) {
                                offset = id + 1;
                                let _ = tx.send(update.clone()).await;
                            }
                        }
                    }
                }
            }
            sleep(Duration::from_millis(100)).await;
        }
    }
}