use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{config::get_config, helpers::APIError};
#[derive(Serialize, Deserialize)]
pub struct Result {
    country: String,
    totalVotes: u32,
}

#[get("/getResultsByCountry")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let res = crate::helpers::get::<Vec<Result>>(
        get_config().calculate_adress,
        "getResultsByCountry".to_string(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(res))
}
