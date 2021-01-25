use crate::helpers::APIError;

pub async fn get(adress: String, endpoint: String) -> Result<String, APIError>
{
    let calculate_path = format!("{}{}", adress, endpoint);
    let resp = reqwest::get(&calculate_path).await?;
    let res = resp.text().await?;
    log::info!("GET BODY CONTENT: {:?}", &res);
    return Ok(res);
}

