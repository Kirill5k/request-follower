use warp::Filter;
use crate::http::health;
use crate::http::proxy;

pub mod http;

#[tokio::main]
async fn main() {
    let routes = proxy::routes().or(health::routes());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
