use crate::Interrupter;
use bytes::Bytes;
use reqwest::{Client, Error};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use warp::http::{HeaderMap, Method, Response, StatusCode};
use warp::path::FullPath;
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
        info!("request headers {:?}", self.headers);
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

    fn reload_on_403(&self) -> bool {
        self.headers.contains_key(X_RELOAD_ON_403)
    }
}

struct ResponseMetadata {
    headers: HeaderMap,
    status: StatusCode,
    body: String,
}

impl ResponseMetadata {
    fn forbidden(err: &str) -> Self {
        ResponseMetadata {
            body: err.to_string(),
            status: StatusCode::FORBIDDEN,
            headers: HeaderMap::new(),
        }
    }

    fn internal_error(err: Error) -> Self {
        ResponseMetadata {
            body: err.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            headers: HeaderMap::new(),
        }
    }

    fn to_response(&self) -> Response<warp::hyper::Body> {
        let mut res = Response::builder();
        for (k, v) in self.headers.iter() {
            res = res.header(k, v);
        }

        res.status(&self.status)
            .body(self.body.clone().into())
            .unwrap()
    }
}

async fn dispatch(
    int: Arc<Interrupter>,
    req_metadata: RequestMetadata,
) -> Result<ResponseMetadata, Error> {
    let res = CLIENT
        .request(req_metadata.method.clone(), &req_metadata.url)
        .query(&Vec::from_iter(req_metadata.query_params.iter()))
        .body(req_metadata.body.clone())
        .headers(req_metadata.sanitised_headers())
        .send()
        .await?;

    let status = res.status();
    let headers = res.headers().clone();

    if status == StatusCode::FORBIDDEN && req_metadata.reload_on_403() {
        int.interrupt();
    }

    res.text().await.map(|body| ResponseMetadata {
        body,
        status,
        headers,
    })
}

pub fn routes(
    int: Arc<Interrupter>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::method()
        .and(warp::path::full())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and(warp::any().map(move || Arc::clone(&int)))
        .and_then(
            |method,
             path: FullPath,
             query,
             headers: HeaderMap,
             body: Bytes,
             int: Arc<Interrupter>| async move {
                let res = match headers.get(X_REROUTE_TO_HEADER) {
                    None => ResponseMetadata::forbidden("Missing X-Reroute-To header"),
                    Some(url) => {
                        let req_metadata = RequestMetadata {
                            method,
                            url: url.to_str().unwrap().to_owned() + path.as_str(),
                            body: String::from_utf8(body.to_vec()).unwrap_or("".to_string()),
                            query_params: query,
                            headers,
                        };
                        dispatch(Arc::clone(&int), req_metadata)
                            .await
                            .unwrap_or_else(ResponseMetadata::internal_error)
                    }
                };
                info!("response {:?}", res.to_response());
                //Ok::<WithStatus<std::string::String>, Rejection>(warp::reply::with_status(res.body, res.status))
                Ok::<Response<warp::hyper::Body>, Rejection>(res.to_response())
            },
        )
}
