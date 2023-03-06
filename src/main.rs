#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::http::health;
use crate::http::proxy;
use time::OffsetDateTime;
use tokio::signal::unix::{signal, SignalKind};
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

    let interrupter = Interrupter::new();

    let stream = signal(SignalKind::terminate());

    let routes = health::routes(interrupter)
        .or(proxy::routes(interrupter))
        .with(warp::log("request_follower"));

    let (_, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async move {
            stream.expect("REASON").recv().await;
        });

    server.await
}
