use crate::{config::get_config, helpers::APIError};
use actix_web::{get, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    name: String,
    country: String,
    candidate: String,
    date: String,
    id: u32
}

#[get("/all")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let res =
        crate::helpers::get::<Vec<Data>>(get_config().raw_data_adress, "all".to_string())
            .await?;

    Ok(HttpResponse::Ok().json(res))
}
