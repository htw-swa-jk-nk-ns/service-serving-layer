use crate::config::get_config;
use actix_web::{post, web, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Vote {
    name: String,
    country: String,
    candidate: String,
    date: String,
}

#[post("/newVote")]
pub async fn exec(info: web::Json<Vec<Vote>>) -> HttpResponse {
    let votes: Vec<Vote> = info
        .iter()
        .map(|vote| Vote {
            candidate: vote.candidate.clone(),
            country: vote.country.clone(),
            name: vote.name.clone(),
            date: vote.date.clone(),
        })
        .collect();

    let res = crate::helpers::post::<Vec<Vote>>(
        get_config().calculate_adress,
        "newVote".to_string(),
        votes,
    )
    .await;
    match res {
        Ok(result) => {
            return HttpResponse::Ok().json(result);
        }
        Err(err) => return HttpResponse::BadRequest().body(err),
    }
}
