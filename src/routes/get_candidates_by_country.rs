use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{config::get_config, helpers::APIError};
#[derive(Serialize, Deserialize)]
pub struct Result {
    country: u32,
    candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize)]
pub struct Candidate {
    name: String,
    total_votes: u32,
}

#[get("/getCandidatesByCountry")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let res = crate::helpers::get(
        get_config().calculate_adress,
        "getCandidatesByCountry".to_string(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(res))
}
