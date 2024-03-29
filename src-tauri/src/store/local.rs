use std::path::PathBuf;
use std::sync::Mutex;
use paperless_rs::ternary;

use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::{AppHandle, Wry};
use tauri_plugin_store::Store;

use super::Error;

pub struct Storage(Mutex<Store<Wry>>);

impl Storage {
    pub fn new(handle: AppHandle, path: PathBuf, mut store: Store<Wry>) -> Result<Self, Error> {
        let app_dir = handle.path_resolver().app_data_dir().ok_or_else(|| Error::IO("Failed to resolve app data dir".into()))?;
        let store_path = app_dir.join(path);

        ternary!(store_path.exists(), {
            store.load().map_err(|e| Error::IO(e.to_string()))?;
        }, ());

        Ok(Self(Mutex::new(store)))
    }

    pub fn fetch<T>(&self, key: &str) -> Result<T, Error>
    where
        T: Serialize + DeserializeOwned + std::fmt::Debug,
    {
        self.on_store(|store| {
            let value = store.get(key);
            let deserialized = value.and_then(|value| {
                let value = value.clone();
                serde_json::from_value(value).ok()
            });

            match deserialized {
                Some(value) => Ok(value),
                None => Err(Error::NotFound),
            }
        })
    }

    pub fn update<T>(&self, key: &str, value: T) -> Result<(), Error>
    where
        T: Serialize + DeserializeOwned,
    {
        self.on_store(|store| {
            let value = serde_json::to_value(value).map_err(|e| Error::Serialize(e.to_string()))?;
            store
                .insert(String::from(key), value)
                .map_err(|e| Error::IO(e.to_string()))
        })
    }

    pub fn save_store(&self) -> Result<(), Error> {
        self.on_store(|store| store.save().map_err(|e| Error::IO(e.to_string())))
    }

    pub fn on_store<F, T>(&self, consumer: F) -> Result<T, Error>
    where
        F: FnOnce(&mut Store<Wry>) -> Result<T, Error>,
    {
        let mut store = self
            .0
            .lock()
            .map_err(|_| Error::IO("Failed to lock store".into()))?;
        consumer(&mut store)
    }
}