use crate::store::Storage;
use crate::utils;
use serde_json::Value;
use tauri::State;

#[tauri::command(async)]
pub async fn fetch_latest_release(
    client: State<'_, reqwest::Client>,
    store: State<'_, Storage>,
) -> Result<String, String> {
    const URL: &str = "https://api.github.com/repos/boul2gom/paperless-rs/releases/latest";
    const ERROR: &str = "Error while fetching the latest release";

    println!("Wrong url used, waiting for the first release to be published...");
    let cached = self::fetch_cached_string(
        client,
        store,
        "latest_release",
        URL,
        ERROR,
        utils::fetch_release,
    )
    .await?;

    Ok(cached)
}

#[tauri::command(async)]
pub async fn fetch_latest_paperless_release(
    client: State<'_, reqwest::Client>,
    store: State<'_, Storage>,
) -> Result<String, String> {
    const URL: &str = "https://api.github.com/repos/paperless-ngx/paperless-ngx/releases/latest";
    const ERROR: &str = "Error while fetching the latest Paperless release";

    let cached = self::fetch_cached_string(
        client,
        store,
        "latest_paperless_release",
        URL,
        ERROR,
        utils::fetch_release,
    )
    .await?;

    Ok(cached)
}

pub async fn fetch_cached_string<F>(
    client: State<'_, reqwest::Client>,
    store: State<'_, Storage>,
    key: &str,
    url: &str,
    error: &str,
    function: F,
) -> Result<String, String>
where
    F: FnOnce(Value) -> Result<String, Box<dyn std::error::Error>>,
{
    match store.fetch::<String>(key) {
        Ok(value) => Ok(value),
        Err(err) => match err {
            crate::store::Error::NotFound | crate::store::Error::ExpiredCache => {
                let response = utils::fetch_json(url, client)
                    .await
                    .map_err(|e| format!("{}: {}", error, e))?;

                let value = function(response).map_err(|e| format!("{}: {}", error, e))?;
                store
                    .update(key, value.clone())
                    .map_err(|e| format!("{}: {:?}", error, e))?;

                Ok(value)
            }
            _ => Err(error.to_string()),
        },
    }
}
