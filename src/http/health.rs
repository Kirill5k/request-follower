use warp::{Filter, Rejection, Reply};
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

async fn get_health_status() -> Result<impl Reply, Rejection> {
    let status = AppStatus::up();
    Ok(warp::reply::json(&status))
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path!("health" / "status"))
        .and(warp::path::end())
        .and_then(get_health_status)
}