use crate::route::base::Route;
use std::sync::Arc;

pub trait LoadBalancer: Send + Sync {
    fn get_route(&self) -> Option<Arc<dyn Route>>;
}