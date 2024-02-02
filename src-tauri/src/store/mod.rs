use chrono::Utc;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Wry;
use tauri_plugin_store::Store;

pub struct Storage(Mutex<Store<Wry>>);

#[derive(Debug)]
pub enum Error {
    NotFound,
    ExpiredCache,

    IO(String),
    Serialize(String),
    Deserialize(String),
}

#[derive(Serialize, Deserialize)]
pub struct Cached<T> {
    pub value: T,
    pub timestamp: u64,
}

impl<T> Cached<T> {
    pub fn new(value: T) -> Self {
        let now = Utc::now().timestamp_millis() as u64;
        Self {
            value,
            timestamp: now,
        }
    }
}

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
                serde_json::from_value::<Cached<T>>(value).ok()
            });

            match deserialized {
                Some(value) => {
                    let now = Utc::now().timestamp_millis() as u64;
                    let timestamp = value.timestamp;

                    if now - timestamp > 1000 * 60 * 60 * 24 {
                        return Err(Error::ExpiredCache);
                    }

                    Ok(value.value)
                }
                None => Err(Error::NotFound),
            }
        })
    }

    pub fn fetch_permanent<T>(&self, key: &str) -> Result<T, Error>
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
            let now = Utc::now().timestamp_millis() as u64;
            let cached = Cached {
                value,
                timestamp: now,
            };

            let value =
                serde_json::to_value(cached).map_err(|e| Error::Serialize(e.to_string()))?;
            store
                .insert(String::from(key), value)
                .map_err(|e| Error::IO(e.to_string()))?;

            store.save().map_err(|e| Error::IO(e.to_string()))
        })
    }

    pub fn update_permanent<T>(&self, key: &str, value: T) -> Result<(), Error>
    where
        T: Serialize + DeserializeOwned,
    {
        self.on_store(|store| {
            let value = serde_json::to_value(value).map_err(|e| Error::Serialize(e.to_string()))?;
            store
                .insert(String::from(key), value)
                .map_err(|e| Error::IO(e.to_string()))?;

            store.save().map_err(|e| Error::IO(e.to_string()))
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
