use reqwest::Response;
use crate::helpers::APIError;

pub async fn get<T>(adress: String, endpoint: String) -> Result<T, APIError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let calculate_path = format!("{}{}", adress, endpoint);
    let resp = reqwest::get(&calculate_path).await?;
    let res = resp.json::<T>().await?;
    return Ok(res);
}
