use std::sync::{Mutex, MutexGuard};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AsyncVec<T> {
    vec: Mutex<Vec<T>>
}

impl<T: Clone> AsyncVec<T> {
    pub fn new(data: Vec<T>) -> AsyncVec<T> {
        AsyncVec { vec: Mutex::new(data) }
    }

    pub fn get(&self) -> Result<Vec<T>, String> {
        let data = self.inner()?.clone();
        Ok(data)
    }

    pub fn update(&self, consumer: impl FnOnce(&mut Vec<T>)) -> Result<(), String> {
        let mut vec = self.inner()?;
        consumer(&mut vec);

        Ok(())
    }

    pub fn inner(&self) -> Result<MutexGuard<Vec<T>>, String> {
        self.vec.lock().map_err(|e| format!("Failed to lock mutex: {:?}", e))
    }
}