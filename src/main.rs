use warp::Filter;
use crate::http::health;

pub mod http;

#[tokio::main]
async fn main() {
    let hello = warp::get()
        .and(warp::path!("hello" / String))
        .and(warp::path::end())
        .map(|name| format!("Hello, {}!", name));

    let routes = hello.or(health::routes());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
