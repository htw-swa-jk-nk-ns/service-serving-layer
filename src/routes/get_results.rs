use crate::{config::get_config, helpers::APIError};
use actix_web::{get, HttpResponse};

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Result {
    candidate: String,
    totalVotes: u32,
}

#[get("/results")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let res =
        crate::helpers::get::<Vec<Result>>(get_config().calculate_adress, "results".to_string())
            .await?;

    Ok(HttpResponse::Ok().json(res))
}
