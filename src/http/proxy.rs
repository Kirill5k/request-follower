use std::collections::HashMap;
use warp::http::{HeaderMap, Method, StatusCode};
use warp::path::FullPath;
use warp::reply::WithStatus;
use warp::{Filter, Rejection, Reply};

const X_REROUTE_TO_HEADER: &str = "X-Reroute-To";

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::method()
        .and(warp::path::full())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .and_then(|method: Method, path: FullPath, query: HashMap<String, String>, headers: HeaderMap| async move {
            match headers.get(X_REROUTE_TO_HEADER) {
                None => Ok::<WithStatus<String>, Rejection>(warp::reply::with_status(
                    "Missing X-Reroute-To header".to_string(),
                    StatusCode::FORBIDDEN,
                )),
                Some(url) => {
                    let full_url = url.to_str().unwrap().to_owned() + path.as_str();
                    Ok::<WithStatus<String>, Rejection>(warp::reply::with_status(
                        format!("{method} {full_url} {:?}\n{:?}", query, headers),
                        StatusCode::OK,
                    ))
                }
            }
        })
}
