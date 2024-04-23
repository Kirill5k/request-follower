use crate::Interrupter;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub mod health;
pub mod proxy;

pub fn routes(
    int: Arc<Interrupter>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    health::routes(Arc::clone(&int))
        .or(proxy::routes(Arc::clone(&int)))
}
