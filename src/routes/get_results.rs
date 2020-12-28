use crate::config::get_config;
use actix_web::{get, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Result {
    candidate: String,
    total_votes: u32,
}

#[get("/getResults")]
pub async fn exec() -> HttpResponse {
    let res = crate::helpers::get::<Vec<Result>>(
        get_config().calculate_adress,
        "results".to_string(),
    )
    .await;
    match res {
        Ok(result) => {
            return HttpResponse::Ok().json(result);
        }
        Err(err) => return HttpResponse::BadRequest().body(err),
    }
}
