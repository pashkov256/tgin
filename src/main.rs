mod base;
mod lb;
mod route;
mod tgin;
mod update;
mod config;

mod utils;

use crate::tgin::Tgin;
use crate::config::setup::{load_config, build_updates, build_route};

fn main() {
    let conf = load_config("tgin.ron");
    println!("Loaded config with {} workers", conf.dark_threads);

    let inputs = build_updates(conf.updates);
    let lb = build_route(conf.route);


    let tgin = Tgin::new(
        inputs, 
        lb, 
        conf.dark_threads, 
        conf.server_port
    );

    tgin.run();
}