use log::{info};

pub async fn post(adress: String, endpoint: String, target: String) -> Result<(), String>
{
    info!("POST BODY: {:?}", &target);
    let client = reqwest::Client::new();
    let calculate_path = format!("{}{}", adress, endpoint);
    let resp = client.post(&calculate_path)
    .header(reqwest::header::CONTENT_TYPE, "application/json").send().await;
    if let Err(err) = resp {
        return Err(format!("{} answered with {}", adress, err.to_string()));
    }

    Ok(())
}
