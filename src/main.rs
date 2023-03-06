#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::http::health;
use crate::http::proxy;
use time::OffsetDateTime;
use warp::Filter;

pub mod http;

#[derive(Clone, Copy, Debug)]
pub struct Interrupter {
    startup_time: OffsetDateTime,
}

impl Interrupter {
    fn new() -> Self {
        Interrupter {
            startup_time: OffsetDateTime::now_utc(),
        }
    }

    fn interrupt(&self) {
        panic!("shutting down the app")
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting request-follower");

    let (_tx, rx) = tokio::sync::oneshot::channel::<()>();

    let interrupter = Interrupter::new();

    let routes = health::routes(interrupter)
        .or(proxy::routes(interrupter))
        .with(warp::log("request_follower"));

    let (_, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async move {
            rx.await.ok();
        });

    server.await
}
