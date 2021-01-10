use crate::config::get_config;
use actix_web::{post, web, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Vote {
    name: String,
    country: String,
    candidate: String,
    date: String,
}

#[post("/vote")]
pub async fn exec(info: web::Json<Vec<Vote>>) -> HttpResponse {
    let votes: Vec<Vote> = info
        .iter()
        .cloned()
        .collect();

    let res = crate::helpers::post::<Vec<Vote>>(
        get_config().raw_data_adress,
        "vote".to_string(),
        votes,
    )
    .await;
    match res {
        Ok(result) => {
            return HttpResponse::Ok().json(result);
        }
        Err(err) => return HttpResponse::InternalServerError().body(err),
    }
}
