use crate::{config::get_config, helpers::APIError};
use actix_web::{HttpResponse, Result, get};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Winner {
    country: String,
    winner: String,
    votes: u32,
}

#[get("/getLandslideVictory")]
pub async fn exec() -> Result<HttpResponse, APIError> {
    let res = crate::helpers::get(
        get_config().calculate_adress,
        "getLandslideVictory".to_string(),
    )
    .await?;

   Ok(HttpResponse::Ok().json(res))
}
