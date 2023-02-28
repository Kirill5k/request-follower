use std::collections::HashMap;
use warp::http::{HeaderMap, Method, StatusCode};
use warp::path::FullPath;
use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::method()
        .and(warp::path::full())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .map(
            |method: Method,
             full_path: FullPath,
             query: HashMap<String, String>,
             headers: HeaderMap| {
                warp::reply::with_status(
                    format!("{method} {:?} {:?}\n{:?}", full_path, query, headers),
                    StatusCode::OK,
                )
            },
        )
}
