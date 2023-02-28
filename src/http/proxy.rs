use std::collections::HashMap;
use warp::http::{HeaderMap, Method, StatusCode};
use warp::path::FullPath;
use warp::{Filter, Rejection, Reply};

const X_REROUTE_TO_HEADER: &str = "X-Reroute-To";

async fn reroute_request(
    method: Method,
    path: FullPath,
    query: HashMap<String, String>,
    headers: HeaderMap,
) -> Result<impl Reply, Rejection> {
    match headers.get(X_REROUTE_TO_HEADER) {
        None => Ok(warp::reply::with_status(
            "Missing X-Reroute-To header".to_string(),
            StatusCode::FORBIDDEN,
        )),
        Some(url) => {
            let full_url = url.to_str().unwrap().to_owned() + path.as_str();
            Ok(warp::reply::with_status(
                format!("{method} {full_url} {:?}\n{:?}", query, headers),
                StatusCode::OK,
            ))
        }
    }
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::method()
        .and(warp::path::full())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .and_then(reroute_request)
}
