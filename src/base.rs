
use async_trait::async_trait;
use serde_json::Value;

use tokio::sync::mpsc::Sender;

use axum::{Router};

use crate::update::base::Updater;


#[async_trait]
pub trait Routeable: Send + Sync {
    async fn process(&self, update: Value);
}

pub trait Serverable {
    fn set_server(&self, server: Router<Sender<Value>>) -> Router<Sender<Value>> {
        server
    }
}

pub trait Printable {
    fn print(&self) -> String;
}

pub trait UpdaterComponent: Updater + Serverable + Printable {}
impl<T: Updater + Serverable + Printable> UpdaterComponent for T {}

pub trait RouteableComponent: Routeable + Serverable + Printable {}
impl<T: Routeable + Serverable + Printable> RouteableComponent for T {}