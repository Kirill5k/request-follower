use bytes::Bytes;
use reqwest::Client;
use std::collections::{HashMap, HashSet};
use warp::http::{HeaderMap, Method, StatusCode};
use warp::path::FullPath;
use warp::reply::WithStatus;
use warp::{Filter, Rejection, Reply};

const X_REROUTE_TO_HEADER: &str = "x-reroute-to";

lazy_static! {
    static ref HEADERS_TO_REMOVE: HashSet<&'static str> = {
        HashSet::from([
            X_REROUTE_TO_HEADER,
            "x-reload-on-403",
            "x-proxied",
            "x-accept-encoding",
            "accept-encoding",
            "host",
            "x-real-ip",
            "x-forwarded-for",
            "x-forwarded-port",
            "x-forwarded-scheme",
            "x-forwarded-host",
            "x-forwarded-proto",
            "x-scheme",
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
                headers.insert(h.clone(), v.clone());
            }
        }
        headers
    }
}

#[derive(Clone)]
struct HttpClient {
    client: Client,
}

impl HttpClient {
    fn new() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }

    async fn dispatch(&self, request_metadata: RequestMetadata) -> Result<(String, u16), String> {
        let response = self
            .client
            .request(request_metadata.method.clone(), &request_metadata.url)
            .query(&Vec::from_iter(request_metadata.query_params.iter()))
            .body(request_metadata.body.clone())
            .headers(request_metadata.sanitised_headers())
            .send()
            .await
            .unwrap();

        let status_code = response.status().as_u16();

        response
            .text()
            .await
            .map(|res_body| (res_body, status_code))
            .map_err(|error| error.to_string())
    }
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let client = HttpClient::new();
    warp::method()
        .and(warp::path::full())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then(
            move |method: Method,
                  path: FullPath,
                  query: HashMap<String, String>,
                  headers: HeaderMap,
                  body: Bytes| {
                let client = client.clone();
                async move {
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
                            let (res_body, res_status) = client
                                .dispatch(req_metadata)
                                .await
                                .unwrap_or(("error".to_string(), 500));
                            Ok::<WithStatus<String>, Rejection>(warp::reply::with_status(
                                res_body,
                                StatusCode::from_u16(res_status).unwrap(),
                            ))
                        }
                    }
                }
            },
        )
}
