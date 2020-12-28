
pub async fn post<T>(adress: String, endpoint: String, target: T) -> Result<(), String>
where
    T: serde::Serialize,
{
    let client = reqwest::Client::new();
    let calculate_path = format!("{}{}", adress, endpoint);
    let resp = client.post(&calculate_path).json(&target).send().await;
    if let Err(err) = resp {
        return Err(format!("{} answered with {}", adress, err.to_string()));
    }

    Ok(())
}
