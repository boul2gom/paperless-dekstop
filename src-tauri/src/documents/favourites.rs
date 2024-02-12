use std::sync::Arc;

use paperless_rs::ternary;
use tauri::State;
use crate::store::{local::Storage, Cached, Error, MemoryCache};

pub struct Favourites;

impl Favourites {
    pub fn load(cache: &MemoryCache, store: &Storage) {
        let result: Result<Vec<u64>, Error> = store.fetch("favourites");
    
        match result {
            Ok(favs) => cache.insert(String::from("favourites"), Cached::Ids(favs)),
            Err(_) => cache.insert(String::from("favourites"), Cached::Ids(Vec::new())),
        }
    }
    
    pub fn save(cache: &MemoryCache, store: &Storage) -> Result<(), Error> {
        let cached = cache.get("favourites").ok_or(Error::NotFound)?;
    
        let data = match cached {
            Cached::Ids(ids) => ids,
            _ => Vec::new(),
        };
        store.update("favourites", data)
    }
}

#[tauri::command(async)]
pub async fn add_to_favourites(cache: State<'_, Arc<MemoryCache>>, id: u64) -> Result<(), String> {
    let mut data = self::get_favourites(cache.clone()).await?;
    
    ternary!(data.contains(&id), return Ok(()), data.push(id));
    cache.insert(String::from("favourites"), Cached::Ids(data));

    Ok(())
}

#[tauri::command(async)]
pub async fn remove_from_favourites(cache: State<'_, Arc<MemoryCache>>, id: u64) -> Result<(), String> {
    let mut data = self::get_favourites(cache.clone()).await?;
    let index = data.iter().position(|&x| x == id).ok_or("Id not found")?;
    
    data.remove(index);
    cache.insert(String::from("favourites"), Cached::Ids(data));

    Ok(())
}

#[tauri::command(async)]
pub async fn get_favourites(cache: State<'_, Arc<MemoryCache>>) -> Result<Vec<u64>, String> {
    match cache.get("favourites") {
        Some(Cached::Ids(ids)) => Ok(ids),
        _ => Ok(Vec::new()),
    }
}