mod lb;
mod route;
mod tgin;
mod update;

use crate::lb::roundrobin::RoundRobin;
use crate::route::base::Route;
use crate::route::webhook::WebhookRoute;
use crate::tgin::Tgin;
use crate::update::webhook::WebhookUpdate;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 1. Инициализация маршрутов (куда отправлять)
    let bot_instance_1 = Arc::new(WebhookRoute::new("http://localhost:8081/bot".to_string()));
    let bot_instance_2 = Arc::new(WebhookRoute::new("http://localhost:8082/bot".to_string()));

    let routes: Vec<Arc<dyn Route>> = vec![bot_instance_1, bot_instance_2];

    // 2. Инициализация Load Balancer
    let lb = Arc::new(RoundRobin::new(routes));

    // 3. Инициализация Tgin
    let mut tgin = Tgin::new(lb, 4);

    // 4. Настройка источника апдейтов (откуда принимать)
    // Tgin будет слушать порт 3000
    tgin.add_update_provider(Box::new(WebhookUpdate::new(3000)));
    
    // Можно добавить LongPoll провайдер
    // tgin.add_update_provider(Box::new(crate::update::longpull::LongPollUpdate::new("TOKEN".to_string())));

    // 5. Старт
    tgin.run().await;
}