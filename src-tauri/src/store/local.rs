use std::sync::Mutex;

use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::Wry;
use tauri_plugin_store::Store;

use super::Error;

pub struct Storage(Mutex<Store<Wry>>);

impl Storage {
    pub fn new(store: Store<Wry>) -> Self {
        Self(Mutex::new(store))
    }

    pub fn fetch<T>(&self, key: &str) -> Result<T, Error>
    where
        T: Serialize + DeserializeOwned,
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