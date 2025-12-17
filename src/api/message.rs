use crate::base::RouteableComponent;

use std::sync::Arc;


pub enum ApiMessage {
    AddRoute(Arc<dyn RouteableComponent>),
}