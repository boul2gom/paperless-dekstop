use paperless_rs::PaperlessClient;
use tauri::State;

pub mod favourites;
pub mod search;

#[tauri::command(async)]
pub async fn document_thumbnail(client: State<'_, PaperlessClient>, id: u64) -> Result<Vec<u8>, String> {
    const ERROR: &str = "Error while fetching thumbnail";

    let response = client.fetch_document_thumbnail(id).await.map_err(|e| format!("{}: {:?}", ERROR, e))?;
    Ok(response.to_vec())
}