use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::config::get_config;
#[derive(Serialize, Deserialize)]
pub struct Result {
    country: String,
    totalVotes: u32,
}

#[get("/getResultsByCountry")]
pub async fn exec() -> HttpResponse {
    let res = crate::helpers::get::<Vec<Result>>(
        get_config().calculate_adress,
        "getResultsByCountry".to_string(),
    )
    .await;
    match res {
        Ok(result) => {
            return HttpResponse::Ok().json(result);
        }
        Err(err) => return HttpResponse::BadRequest().body(err),
    }
}
