use std::sync::Arc;

use tauri::State;
use crate::{store::{Cached, MemoryCache}, utils};

#[tauri::command(async)]
pub async fn fetch_latest_release(
    client: State<'_, reqwest::Client>,
    cache: State<'_, Arc<MemoryCache>>,
) -> Result<String, String> {
    const URL: &str = "https://api.github.com/repos/boul2gom/paperless-rs/releases/latest";
    const ERROR: &str = "Error while fetching the latest release";

    println!("Wrong url used, waiting for the first release to be published...");
    let cached = self::fetch_cached_string(
        client,
        cache,
        "latest_release",
        URL,
        ERROR,
    )
    .await?;

    Ok(cached)
}

#[tauri::command(async)]
pub async fn fetch_latest_paperless_release(
    client: State<'_, reqwest::Client>,
    cache: State<'_, Arc<MemoryCache>>,
) -> Result<String, String> {
    const URL: &str = "https://api.github.com/repos/paperless-ngx/paperless-ngx/releases/latest";
    const ERROR: &str = "Error while fetching the latest Paperless release";

    let cached = self::fetch_cached_string(
        client,
        cache,
        "latest_paperless_release",
        URL,
        ERROR,
    )
    .await?;

    Ok(cached)
}

pub async fn fetch_cached_string(
    client: State<'_, reqwest::Client>,
    cache: State<'_, Arc<MemoryCache>>,
    key: &str,
    url: &str,
    error: &str,
) -> Result<String, String> {
    let cached = cache.get(key).map(|cached| match cached {
        Cached::Version(version) => Ok(version),
        _ => Err::<String, String>("Invalid cached type".into()),
    });

    match cached {
        Some(Ok(cached)) => Ok(cached),
        Some(Err(_)) | None => {
            let response = utils::fetch_json(url, client).await.map_err(|e| format!("{}: {}", error, e))?;
            let release = self::extract_release(response).map_err(|e| format!("{}: {}", error, e))?;

            
            cache.insert(key.to_string(), Cached::Version(release.clone()));
            Ok(release)
        }
    }
}

pub fn extract_release(value: serde_json::Value) -> Result<String, String> {
    let tag_name = value["tag_name"]
        .as_str()
        .ok_or_else(|| "No tag_name in the response".to_string())?;
    let version = tag_name.trim_start_matches('v');
    Ok(version.to_string())
}