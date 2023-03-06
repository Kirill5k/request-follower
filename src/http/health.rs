use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};
use crate::Interrupter;

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

pub fn routes(interrupter: Interrupter) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path!("health" / "status"))
        .and(warp::path::end())
        .and(warp::any().map(move || interrupter.clone()))
        .map(move |interrupter: Interrupter| {
            warp::reply::with_status(
                warp::reply::json(&AppStatus::up(interrupter.startup_time)),
                StatusCode::OK,
            )
        })
}
