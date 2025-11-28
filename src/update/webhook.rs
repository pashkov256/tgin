use crate::update::base::UpdateProvider;
use async_trait::async_trait;
use axum::{extract::State, routing::post, Json, Router};
use serde_json::Value;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

pub struct WebhookUpdate {
    addr: SocketAddr,
}

impl WebhookUpdate {
    pub fn new(port: u16) -> Self {
        Self {
            addr: SocketAddr::from(([0, 0, 0, 0], port)),
        }
    }
}

#[async_trait]
impl UpdateProvider for WebhookUpdate {
    async fn start(&self, tx: Sender<Value>) {
        let app = Router::new()
            .route("/", post(handler))
            .with_state(tx);

        let listener = tokio::net::TcpListener::bind(self.addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

async fn handler(State(tx): State<Sender<Value>>, Json(update): Json<Value>) {
    let _ = tx.send(update).await;
}