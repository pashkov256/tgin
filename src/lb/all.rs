
use crate::base::{Routeable, RouteableComponent, Serverable, Printable};

use tokio::sync::{mpsc::Sender, RwLock};
use axum::{Router};

use std::sync::Arc;


use async_trait::async_trait;

use serde_json::{Value, json};

pub struct AllLB {
    routes: RwLock<Vec<Arc<dyn RouteableComponent>>>
}

impl AllLB {
    pub fn new(routes: Vec<Arc<dyn RouteableComponent>>) -> Self {
        Self {
            routes: RwLock::new(routes),
        }
    }
}

#[async_trait]
impl Routeable for AllLB {
    async fn process(&self, update: Value) {
        let routes = self.routes.read().await;
        for route in routes.iter() {
            let route = route.clone();
            let update = update.clone();

            tokio::spawn(async move {
                route.process(update).await;
            });
        }
    }
}

#[async_trait]
impl Serverable for AllLB {
    async fn set_server(&self, mut router: Router<Sender<Value>>) -> Router<Sender<Value>> {
        let routes = self.routes.read().await;
        for route in routes.iter() {
            router = route.set_server(router).await;
        }
        router
    }
}

#[async_trait]
impl Printable for AllLB {
    async fn print(&self) -> String {
        let routes = self.routes.read().await;
        let mut text = String::from("LOAD BALANCER All\n\n");

        for route in routes.iter() {
            text.push_str(&format!("{}\n\n", route.print().await));
        }
        text
    }

    async fn json_struct(&self) -> Value {
        let routes = self.routes.read().await;
        let mut routes_json: Vec<Value> = Vec::new();
        for route in routes.iter() {
            routes_json.push(route.json_struct().await);
        }

        json!({
            "type": "load-balancer",
            "name": "all",
            "routes": routes_json
        })
    }


}