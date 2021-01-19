use crate::config::get_config;
use actix_web::{post, web, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Vote {
    pub name: String,
    pub country: String,
    pub candidate: String,
}

#[post("/vote")]
pub async fn exec(info: web::Json<Vote>) -> HttpResponse {

    let res = crate::helpers::post::<Vote>(
        get_config().raw_data_adress,
        "vote".to_string(),
        info.into_inner(),
    )
    .await;
    match res {
        Ok(_) => {
            return HttpResponse::Ok().finish();
        }
        Err(err) => return HttpResponse::InternalServerError().body(err),
    }
}
