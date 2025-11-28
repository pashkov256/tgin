use crate::lb::base::LoadBalancer;
use crate::route::base::Route;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct RoundRobin {
    routes: Vec<Arc<dyn Route>>,
    current: AtomicUsize,
}

impl RoundRobin {
    pub fn new(routes: Vec<Arc<dyn Route>>) -> Self {
        Self {
            routes,
            current: AtomicUsize::new(0),
        }
    }
}

impl LoadBalancer for RoundRobin {
    fn get_route(&self) -> Option<Arc<dyn Route>> {
        if self.routes.is_empty() {
            return None;
        }
        let current = self.current.fetch_add(1, Ordering::Relaxed);
        let index = current % self.routes.len();
        Some(self.routes[index].clone())
    }
}