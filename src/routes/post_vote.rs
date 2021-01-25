use crate::config::get_config;
use actix_web::{post, web, HttpResponse};

use serde::{Deserialize, Serialize};

#[post("/vote")]
pub async fn exec(info: String) -> HttpResponse {

    let res = crate::helpers::post(
        get_config().raw_data_adress,
        "vote".to_string(),
        info,
    )
    .await;
    match res {
        Ok(_) => {
            return HttpResponse::Ok().finish();
        }
        Err(err) => return HttpResponse::InternalServerError().body(err),
    }
}
