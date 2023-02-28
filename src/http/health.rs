use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use warp::{Filter, Rejection, Reply};

#[derive(Serialize, Deserialize, Debug)]
struct AppStatus {
    status: String,
    #[serde(with = "time::serde::rfc3339")]
    startup_time: OffsetDateTime,
}

impl AppStatus {
    fn up(startup_time: OffsetDateTime) -> AppStatus {
        AppStatus {
            status: String::from("up"),
            startup_time,
        }
    }
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let startup_time = OffsetDateTime::now_utc();
    warp::get()
        .and(warp::path!("health" / "status"))
        .and(warp::path::end())
        .map(move || warp::reply::json(&AppStatus::up(startup_time)))
}
