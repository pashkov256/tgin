use crate::base::{Serverable, Printable};
use crate::update::base::Updater;

use crate::utils::defaults::TELEGRAM_TOKEN_REGEX;

use async_trait::async_trait;
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};

use reqwest::Client;

use tokio::sync::mpsc::Sender;

use regex::Regex;

pub struct RegistrationWebhookConfig {
    public_ip: String,
    client: Client,
    set_webhook_url: String,

    token_regex: Regex,

}

impl RegistrationWebhookConfig {
    pub fn new(token: String, public_ip: String) -> Self {
        Self {
            public_ip,
            client: Client::new(),
            set_webhook_url: format!("https://api.telegram.org/bot{}/setWebhook", token),
            token_regex: Regex::new(TELEGRAM_TOKEN_REGEX).unwrap(),
        }
    }

    pub fn set_client(&mut self, client: Client) {
        self.client = client;
    }

    pub fn set_webhook_url(&mut self, set_webhook_url: String) {
        self.set_webhook_url = set_webhook_url;
    }

    pub fn set_regex_token(&mut self, regex: Regex) {
        self.token_regex = regex;
    }

}


pub struct WebhookUpdate {
    path: String,
    registration: Option<RegistrationWebhookConfig>, 
}



impl WebhookUpdate {
    pub fn new(path: String) -> Self {
        Self { path, registration: None }
    }


    pub async fn register_webhook(&self, config: &RegistrationWebhookConfig) {
        let full_url = format!("{}{}", config.public_ip.trim_end_matches('/'), self.path);

        let params = json!({ "url": full_url });

        match config.client.post(&config.set_webhook_url).json(&params).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("Webhook set successfully for path: {}", self.path);
                } else {
                    eprintln!("Failed to set webhook. Status: {}", resp.status());
                }
            }
            Err(e) => eprintln!("Network error setting webhook: {}", e),
        }
    }


}

#[async_trait]
impl Updater for WebhookUpdate {
    async fn start(&self, _tx: Sender<Value>) {
        if let Some(config) = &self.registration {
            self.register_webhook(config).await;
        } else {
            println!("Webhook started in passive mode (no auto-registration) for {}", self.path);
        }
    }
}


#[async_trait]
impl Serverable for WebhookUpdate {
    async fn set_server(&self, router: Router<Sender<Value>>) -> Router<Sender<Value>> {
        router.route(&self.path, post(handler))
    }
}


async fn handler(State(tx): State<Sender<Value>>, Json(update): Json<Value>) {
    let _ = tx.send(update).await;
}


#[async_trait]
impl Printable for WebhookUpdate {
    async fn print(&self) -> String {
        let reg_text = match &self.registration {
            Some(reg)  => format!("REGISTRATED ON {}", &reg.token_regex.replace_all(&reg.set_webhook_url, "#####")),
            None => "".to_string()
        };
        format!("webhook: 0.0.0.0{} {}", self.path, reg_text)
    }
}