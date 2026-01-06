#![cfg(test)]

use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use crate::base::{Routeable, RouteableComponent, Printable, Serverable};
use tokio::sync::mpsc::Sender;
use axum::Router;

pub struct MockCallsRoute {
    pub id: String,
    pub calls: Arc<Mutex<Vec<Value>>>,
}

impl MockCallsRoute {
    pub fn new(id: &str) -> Self {
        Self { 
            id: id.to_string(), 
            calls: Arc::new(Mutex::new(vec![])) 
        }
    }
    
    pub async fn count(&self) -> usize {
        self.calls.lock().await.len()
    }

    pub async fn get_calls(&self) -> Vec<Value> {
        self.calls.lock().await.clone()
    }
}

#[async_trait]
impl Routeable for MockCallsRoute {
    async fn process(&self, update: Value) {
        self.calls.lock().await.push(update);
    }
}

#[async_trait]
impl Printable for MockCallsRoute {
    async fn print(&self) -> String {
        format!("MockRoute({})", self.id)
    }
    async fn json_struct(&self) -> Value {
        json!({ "type": "mock", "id": self.id })
    }
}

#[async_trait]
impl Serverable for MockCallsRoute {
}

