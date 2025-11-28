use crate::lb::base::LoadBalancer;
use crate::update::base::UpdateProvider;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct Tgin {
    update_providers: Vec<Box<dyn UpdateProvider>>,
    lb: Arc<dyn LoadBalancer>,
    workers: usize,
}

impl Tgin {
    pub fn new(lb: Arc<dyn LoadBalancer>, workers: usize) -> Self {
        Self {
            update_providers: Vec::new(),
            lb,
            workers,
        }
    }

    pub fn add_update_provider(&mut self, provider: Box<dyn UpdateProvider>) {
        self.update_providers.push(provider);
    }

    pub async fn run(self) {
        let (tx, mut rx) = mpsc::channel::<Value>(10000);

        for provider in self.update_providers {
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                provider.start(tx_clone).await;
            });
        }

        // Drop original tx so channel closes when producers end (though servers run forever)
        drop(tx);

        let lb = self.lb.clone();
        
        // Main Event Loop
        while let Some(update) = rx.recv().await {
            if let Some(route) = lb.get_route() {
                tokio::spawn(async move {
                    route.send(update).await;
                });
            }
        }
    }
}