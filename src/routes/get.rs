use reqwest::Request;
use serde::{Serialize, Deserialize};
use actix_web::{get, HttpResponse, HttpServer, Responder, client::ConnectError, guard, web};
use std::collections::HashMap;
use crate::config::get_config;

pub async fn get<T>(adress: String, endpoint: String) -> Result<T, String> 
where T: for<'de> serde::Deserialize<'de> {
    let calculate_path = format!("{}{}", adress, endpoint);
    let resp = reqwest::get(&calculate_path).await;
    if let Err(err) = resp {
        return Err(format!("{} answered with {}", adress, err.to_string()))
    }

    let resp_result = resp.unwrap();

    let mut res ;
    
    if !resp_result.status().is_success() {
       return Err(
            format!("{} answered with {}", adress, resp_result.status())
        )
    }

    res = resp_result.json::<T>().await;
    if let Err(err) = res {
        return Err(
            format!("Cannot parse JSON from {} with Error {}", adress, err.to_string())
        )
    }
    return Ok(res.unwrap());
}