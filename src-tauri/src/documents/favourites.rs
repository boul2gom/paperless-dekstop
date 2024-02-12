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

#[tauri::command]
pub fn add_to_favourites(cache: State<Arc<MemoryCache>>, id: u64) -> () {
    let mut data = self::get_favourites(cache.clone());
    
    ternary!(data.contains(&id), return, data.push(id));
    cache.insert(String::from("favourites"), Cached::Ids(data));
}

#[tauri::command]
pub fn remove_from_favourites(cache: State<Arc<MemoryCache>>, id: u64) -> Result<(), String> {
    let mut data = self::get_favourites(cache.clone());
    let index = data.iter().position(|&x| x == id).ok_or("Id not found")?;
    
    data.remove(index);
    cache.insert(String::from("favourites"), Cached::Ids(data));

    Ok(())
}

#[tauri::command]
pub fn get_favourites(cache: State<Arc<MemoryCache>>) -> Vec<u64> {
    match cache.get("favourites") {
        Some(Cached::Ids(ids)) => ids,
        _ => Vec::new(),
    }
}