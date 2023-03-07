use crate::Interrupter;
use bytes::Bytes;
use reqwest::{Client, Error};
use std::collections::{HashMap, HashSet};
use warp::http::{HeaderMap, Method, StatusCode};
use warp::path::FullPath;
use warp::reply::WithStatus;
use warp::{Filter, Rejection, Reply};

const X_REROUTE_TO_HEADER: &str = "x-reroute-to";
const X_ACCEPT_ENCODING: &str = "x-accept-encoding";
const X_RELOAD_ON_403: &str = "x-reload-on-403";

lazy_static! {
    static ref CLIENT: Client = Client::new();
    static ref HEADERS_TO_REMOVE: HashSet<&'static str> = {
        HashSet::from([
            X_REROUTE_TO_HEADER,
            X_ACCEPT_ENCODING,
            X_RELOAD_ON_403,
            "host",
            "x-proxied",
            "x-real-ip",
            "x-scheme",
            "x-forwarded-for",
            "x-forwarded-port",
            "x-forwarded-scheme",
            "x-forwarded-host",
            "x-forwarded-proto",
        ])
    };
}

struct RequestMetadata {
    method: Method,
    url: String,
    query_params: HashMap<String, String>,
    headers: HeaderMap,
    body: String,
}

impl RequestMetadata {
    fn sanitised_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for (h, v) in self.headers.iter() {
            if !HEADERS_TO_REMOVE.contains(h.as_str()) {
                headers.insert(&*h, v.into());
            }
        }
        if let Some(hv) = self.headers.get(X_ACCEPT_ENCODING) {
            headers.insert(reqwest::header::ACCEPT_ENCODING, hv.into());
        }
        headers
    }
}

struct ResponseMetadata {
    headers: HeaderMap,
    status: StatusCode,
    body: String
}

impl ResponseMetadata {
    fn error(err: String) -> Self {
        ResponseMetadata {
            body: err,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            headers: HeaderMap::new()
        }
    }
}

async fn dispatch(request_metadata: RequestMetadata) -> Result<ResponseMetadata, Error> {
    let res = CLIENT
        .request(request_metadata.method.clone(), &request_metadata.url)
        .query(&Vec::from_iter(request_metadata.query_params.iter()))
        .body(request_metadata.body.clone())
        .headers(request_metadata.sanitised_headers())
        .send()
        .await?;

    let res_status = res.status();
    let res_headers = res.headers().clone();
    res.text().await.map(|res_body| {
        ResponseMetadata {
            body: res_body,
            status: res_status,
            headers: res_headers
        }
    })
}

pub fn routes(int: Interrupter) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    println!("{}", int.startup_time);
    warp::method()
        .and(warp::path::full())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then(
            |method, path: FullPath, query, headers: HeaderMap, body: Bytes| async move {
                match headers.get(X_REROUTE_TO_HEADER) {
                    None => Ok::<WithStatus<String>, Rejection>(warp::reply::with_status(
                        "Missing X-Reroute-To header".to_string(),
                        StatusCode::FORBIDDEN,
                    )),
                    Some(url) => {
                        let req_metadata = RequestMetadata {
                            method,
                            url: url.to_str().unwrap().to_owned() + path.as_str(),
                            body: String::from_utf8(body.to_vec()).unwrap_or("".to_string()),
                            query_params: query,
                            headers,
                        };
                        let res = dispatch(req_metadata).await.unwrap_or_else(|err| ResponseMetadata::error(err.to_string()));
                        Ok::<WithStatus<String>, Rejection>(warp::reply::with_status(
                            res.body, res.status,
                        ))
                    }
                }
            },
        )
}
