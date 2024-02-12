pub mod types;

pub async fn fetch_data(
    url: &str,
    client: tauri::State<'_, reqwest::Client>,
) -> Result<String, Box<dyn std::error::Error>> {
    let resp = client.get(url).send().await?;
    let text = resp.text().await?;
    Ok(text)
}

pub async fn fetch_json(
    url: &str,
    client: tauri::State<'_, reqwest::Client>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let resp = client.get(url).send().await?;
    let json = resp.json().await?;
    Ok(json)
}