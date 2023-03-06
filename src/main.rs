#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::http::health;
use crate::http::proxy;
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use warp::Filter;

pub mod http;

#[derive(Clone, Debug)]
pub struct Interrupter {
    startup_time: OffsetDateTime,
    sender: Sender<()>,
}

impl Interrupter {
    fn new(sender: Sender<()>) -> Self {
        Interrupter {
            startup_time: OffsetDateTime::now_utc(),
            sender,
        }
    }

    fn interrupt(&self) {
        self.sender.try_send(()).unwrap();
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting request-follower");

    let (tx, mut rx) = mpsc::channel::<()>(1);
    let interrupter = Interrupter::new(tx);

    let routes = health::routes(interrupter.clone())
        .or(proxy::routes(interrupter.clone()))
        .with(warp::log("request_follower"));

    let (_, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async move {
            rx.recv().await;
            info!("received termination signal")
        });

    server.await
}
