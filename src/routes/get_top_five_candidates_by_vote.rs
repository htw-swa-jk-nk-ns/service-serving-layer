use crate::{config::get_config, helpers::APIError};
use actix_web::{get, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Result {
    candidate: String,
    votes: u32,
}

#[get("/getTopFiveCandidatesByVote")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let res = crate::helpers::get(
        get_config().calculate_adress,
        "getTopFiveCandidatesByVote".to_string(),
    )
    .await?;
    Ok(HttpResponse::Ok().set(actix_web::http::header::ContentType::json()).body(res))
}
