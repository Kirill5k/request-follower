use warp::Filter;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct AppStatus {
    status: String
}

impl AppStatus {
    fn up() -> AppStatus {
        AppStatus {
            status: String::from("up")
        }
    }
}

#[tokio::main]
async fn main() {
    let hello = warp::get()
        .and(warp::path!("hello" / String))
        .map(|name| format!("Hello, {}!", name));

    let health = warp::get()
        .and(warp::path!("health" / "status"))
        .map(|| r#"{"status":"up"}"#);

    let routes = hello.or(health);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
