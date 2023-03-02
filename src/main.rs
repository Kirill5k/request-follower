#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::http::health;
use crate::http::proxy;
use warp::Filter;

pub mod http;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting request-follower");

    let routes = health::routes().or(proxy::routes()).with(warp::log("request_follower"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
