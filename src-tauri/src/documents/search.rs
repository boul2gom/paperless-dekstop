use std::time::Instant;

use paperless_rs::endpoint::documents::Document;
use paperless_rs::PaperlessClient;
use tauri::State;

#[tauri::command(async)]
pub async fn documents_query(client: State<'_, PaperlessClient>, query: String) -> Result<Vec<Document>, String> {
    const ERROR: &str = "Error while querying documents";

    if query.is_empty() {
        let response = Vec::new();
        return Ok(response);
    }

    let start = Instant::now();
    let response = client.search_documents(&query).await.map_err(|e| format!("{}: {:?}", ERROR, e))?;
    println!("Queryed '{}' document in {:?}", response.count, start.elapsed());
    
    Ok(response.results)
}