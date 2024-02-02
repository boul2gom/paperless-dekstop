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

pub fn fetch_release(value: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let tag_name = value["tag_name"]
        .as_str()
        .ok_or_else(|| "No tag_name in the response".to_string())?;
    let version = tag_name.trim_start_matches('v');
    Ok(version.to_string())
}
