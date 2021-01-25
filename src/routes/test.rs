use crate::{helpers::{APIError, get}};
use actix_web::{HttpResponse, get, http::header::ContentType};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    info: String
}

#[get("/test")]
pub async fn exec() -> actix_web::Result<HttpResponse, APIError> {
    let content = get("https://jsonplaceholder.typicode.com/".to_string(), "todos/1".to_string()).await?;
    Ok(HttpResponse::Ok().set(ContentType::json()).body(content))
}
