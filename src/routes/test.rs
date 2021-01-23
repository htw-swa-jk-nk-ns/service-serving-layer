use crate::{helpers::APIError};
use actix_web::{get, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    info: String
}

#[get("/test")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let res = Info{info: "I work!".to_string()};

    Ok(HttpResponse::Ok().json(res))
}
