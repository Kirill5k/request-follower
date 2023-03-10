#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::config::AppConfig;
use crate::http::health;
use crate::http::proxy;
use time::{Duration, OffsetDateTime};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use warp::Filter;

pub mod config;
pub mod http;

#[derive(Clone, Debug)]
pub struct Interrupter {
    startup_time: OffsetDateTime,
    sender: Sender<()>,
    initial_delay: Duration,
}

impl Interrupter {
    fn new(sender: Sender<()>) -> Self {
        Interrupter {
            startup_time: OffsetDateTime::now_utc(),
            sender,
            initial_delay: Duration::minutes(30),
        }
    }

    fn interrupt(&self) {
        let diff = OffsetDateTime::now_utc() - self.startup_time;
        if diff > self.initial_delay {
            info!("sending termination signal");
            self.sender.try_send(()).unwrap();
        } else {
            info!(
                "termination is delayed as app has started {}min ago",
                diff.whole_minutes()
            );
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting request-follower app");
    let config = AppConfig::new().unwrap();
    info!("loaded config {:?}", config);

    let (tx, mut rx) = mpsc::channel::<()>(1);
    let interrupter = Interrupter::new(tx);

    let routes = health::routes(interrupter.clone())
        .or(proxy::routes(interrupter.clone()))
        .with(warp::log("request_follower"));

    info!("starting web-server on port {}", config.server.port);
    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(
        ([0, 0, 0, 0], config.server.port),
        async move {
            rx.recv().await;
            info!("received termination signal")
        },
    );

    server.await
}
