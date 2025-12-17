use axum::{Router, routing::post, Json};
use serde_json::{Value};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};


use crate::base::Serverable;
use crate::api::message::ApiMessage;

use crate::api::methods;




struct Api {
    base_path: String,
    tx: Sender<ApiMessage>,
    rx: Receiver<ApiMessage>,
}


impl Api {
    pub fn new(base_path: String) -> Self {
        let (tx, mut rx) = mpsc::channel::<ApiMessage>(100);
        Self { 
            base_path,
            tx, 
            rx
        }
    }






}


impl Serverable for Api {
    fn set_server(&self, main_router: Router<Sender<Value>>) -> Router<Sender<Value>> {
        let router = Router::new()
            .route(&self.base_path, post(methods::add_route))
            .with_state(self.tx.clone());


        main_router.nest(&self.base_path, router)

    }
}












