use crate::{config::get_config, helpers::APIError};
use actix_web::{get, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Result {
    candidate: String,
    total_votes: u32,
}

#[get("/getResults")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let res =
        crate::helpers::get::<Vec<Result>>(get_config().calculate_adress, "results".to_string())
            .await?;

    Ok(HttpResponse::Ok().json(res))
}
