pub mod health;
pub mod proxy;

use crate::Interrupter;
use warp::{Filter, Rejection, Reply};

pub fn routes(int: Interrupter) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    health::routes(int.clone())
        .or(proxy::routes(int.clone()))
        .with(warp::log("request_follower"))
}
