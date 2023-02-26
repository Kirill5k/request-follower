use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let health = warp::path!("health" / "status")
        .map(|| r#"{"status":"up"}"#);

    let routes = warp::get().and(hello.or(health));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
