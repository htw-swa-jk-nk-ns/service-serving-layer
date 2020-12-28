use crate::config::get_config;
use actix_web::{get, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Result {
    candidate: String,
    votes: u32,
}

#[get("/getTopFiveCandidatesByVote")]
pub async fn exec() -> HttpResponse {
    let res = crate::helpers::get::<Vec<Result>>(
        get_config().calculate_adress,
        "getTopFiveCandidatesByVote".to_string(),
    )
    .await;
    match res {
        Ok(result) => {
            return HttpResponse::Ok().json(result);
        }
        Err(err) => return HttpResponse::BadRequest().body(err),
    }
}
