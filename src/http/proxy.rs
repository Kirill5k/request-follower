use warp::http::{HeaderMap, Method};
use warp::{Filter, Rejection, Reply};
use warp::path::FullPath;
use std::collections::HashMap;

pub fn routes() -> impl Filter<Extract=(impl Reply, ), Error=Rejection> + Clone {
    warp::method()
        .and(warp::path::full())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .map(|method: Method, full_path: FullPath, query: HashMap<String, String>, headers: HeaderMap| {
            format!("{method} {:?} {:?}\n{:?}", full_path, query, headers)
        })
}
