use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path!("hello" / String))
        .and(warp::path::end())
        .map(|name| format!("Hello, {}!", name))
}