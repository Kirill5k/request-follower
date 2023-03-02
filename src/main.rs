#[macro_use]
extern crate lazy_static;

use crate::http::health;
use crate::http::proxy;
use warp::Filter;

pub mod http;

#[tokio::main]
async fn main() {
    let routes = health::routes().or(proxy::routes());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
