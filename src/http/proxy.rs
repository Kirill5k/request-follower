use warp::{Filter, Rejection, Reply};
use warp::http::{HeaderMap, Method};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("hello" / String)
        .and(warp::path::end())
        .and(warp::method())
        .and(warp::header::headers_cloned())
        .map(|name: String, method: Method, headers: HeaderMap| {
            format!("Hello, {}!\nMethod: {}\nHeaders: {:?}", name, method, headers)
        })
}