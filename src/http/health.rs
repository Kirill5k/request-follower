use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
struct AppStatus {
    status: String,
    #[serde(with = "time::serde::rfc3339")]
    startup_time: OffsetDateTime
}

impl AppStatus {
    fn up(startup_time: OffsetDateTime) -> AppStatus {
        AppStatus {
            status: String::from("up"),
            startup_time
        }
    }
}

async fn get_health_status(startup_time: OffsetDateTime) -> Result<impl Reply, Rejection> {
    let status = AppStatus::up(startup_time);
    Ok(warp::reply::json(&status))
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let startup_time = OffsetDateTime::now_utc();
    warp::get()
        .and(warp::path!("health" / "status"))
        .and(warp::path::end())
        .and_then(move || get_health_status(startup_time))
}