use crate::Interrupter;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

#[derive(Serialize, Deserialize, Debug)]
struct AppStatus {
    status: String,
    #[serde(with = "time::serde::rfc3339")]
    startup_time: OffsetDateTime,
    up_time: Duration
}

impl AppStatus {
    fn up(startup_time: OffsetDateTime) -> AppStatus {
        AppStatus {
            status: String::from("up"),
            startup_time,
            up_time: OffsetDateTime::now_utc() - startup_time
        }
    }

    fn down(startup_time: OffsetDateTime) -> AppStatus {
        AppStatus {
            status: String::from("down"),
            startup_time,
            up_time: OffsetDateTime::now_utc() - startup_time
        }
    }
}

pub fn routes(int: Interrupter) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let endpoint_base = warp::path!("health" / "status")
        .and(warp::path::end())
        .and(warp::any().map(move || int.clone()));

    let get_status = endpoint_base
        .clone()
        .and(warp::get())
        .map(move |interrupter: Interrupter| {
            warp::reply::with_status(
                warp::reply::json(&AppStatus::up(interrupter.startup_time)),
                StatusCode::OK,
            )
        });

    let interrupt_app =
        endpoint_base
            .clone()
            .and(warp::delete())
            .map(move |interrupter: Interrupter| {
                interrupter.interrupt();
                warp::reply::with_status(
                    warp::reply::json(&AppStatus::down(interrupter.startup_time)),
                    StatusCode::IM_A_TEAPOT,
                )
            });

    get_status.or(interrupt_app)
}
