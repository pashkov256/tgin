
use crate::base::{Routeable, RouteableComponent, Serverable};

use tokio::sync::mpsc::Sender;
use axum::{Router};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;


use async_trait::async_trait;

use serde_json::Value;

pub struct AllLB {
    routes: Vec<Arc<dyn RouteableComponent>>,
}

impl AllLB {
    pub fn new(routes: Vec<Arc<dyn RouteableComponent>>) -> Self {
        Self {
            routes,
        }
    }
}

#[async_trait]
impl Routeable for AllLB {
    async fn process(&self, update: Value) {
        for route in &self.routes {

            let route = route.clone();
            let update = update.clone();

            tokio::spawn(async move {
                route.process(update).await;
            });
        }
    }
}

impl Serverable for AllLB {
    fn set_server(&self, mut router: Router<Sender<Value>>) -> Router<Sender<Value>> {
        for route in &self.routes {
            router = route.set_server(router);
        }
        router
    }
}