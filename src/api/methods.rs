
use axum::{Router, routing::post, extract::State, Json};
use tokio::sync::mpsc::Sender;
use std::sync::Arc;

use crate::api::schemas::{RouteType};
use crate::api::message::ApiMessage;

use crate::route::webhook::WebhookRoute;








pub async fn add_route(State(tx): State<Sender<ApiMessage>>, Json(data): Json<RouteType>)  {
    match data {
        RouteType::Longpull(route) => {


        },
        RouteType::Webhook(route) => {
            let update = WebhookRoute::new(route.url);
            let update = Arc::new(update);
            let _ = tx.send(ApiMessage::AddRoute(update)).await;
        }
    }

}