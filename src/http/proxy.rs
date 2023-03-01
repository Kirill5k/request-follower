use bytes::Bytes;
use reqwest::Client;
use std::collections::HashMap;
use warp::http::{HeaderMap, Method, StatusCode};
use warp::path::FullPath;
use warp::reply::WithStatus;
use warp::{Filter, Rejection, Reply};

const X_REROUTE_TO_HEADER: &str = "X-Reroute-To";

#[derive(Debug)]
struct RequestMetadata {
    method: Method,
    url: String,
    query_params: HashMap<String, String>,
    headers: HeaderMap,
    body: String,
}

impl RequestMetadata {
    fn sanitised_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for (h, v) in self.headers.iter() {
            headers.insert(h.as_str().to_owned(), v.to_str().unwrap().to_owned());
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
        Ok((
            format!(
                "{:?}\n{:?}",
                request_metadata,
                request_metadata.sanitised_headers()
            ),
            200,
        ))
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
