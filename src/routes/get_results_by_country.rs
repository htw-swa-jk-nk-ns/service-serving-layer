use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{config::get_config, helpers::APIError};
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Result {
    country: String,
    totalVotes: u32,
}

#[get("/votesByCountry")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let res = crate::helpers::get(
        get_config().calculate_adress,
        "getResultsByCountry".to_string(),
    )
    .await?;
    Ok(HttpResponse::Ok().set(actix_web::http::header::ContentType::json()).body(res))
}
