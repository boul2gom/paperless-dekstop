#[allow(dead_code)]
#[tauri::command(async)]
pub async fn documents_query(_query: String) -> Result<Vec<String>, String> {
    const ERROR: &str = "Error while querying documents";

    let mut response = Vec::new();
    response.push("First document".to_string());
    response.push("Second document".to_string());
    response.push("Third document".to_string());

    Ok(response)
}
