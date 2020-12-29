use crate::{config::get_config, helpers::APIError};
use actix_web::{get, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Result {
    country: String,
    total_votes: u32,
    top_candidates: Vec<Candidate>,
    difference: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Candidate {
    name: String,
    votes: u32,
}

#[get("/getClosestVoteDifference")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let res = crate::helpers::get::<Result>(
        get_config().calculate_adress,
        "getClosestVoteDifference".to_string(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(res))
}
