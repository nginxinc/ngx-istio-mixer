use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard };
use super::cache_elem::CacheElem;

// simple LRU Cache, index by hash


pub struct LRUCache {

    values: Arc<Mutex<HashMap<u64,CacheElem>>>
}

impl  LRUCache {

    pub fn new() -> LRUCache  {
        LRUCache{
            values: Arc::new(Mutex::new(HashMap::new()))
        }
    }


    pub fn get(&self, key: u64) -> Option<CacheElem> {
        let map: MutexGuard<HashMap<u64,CacheElem>> = self.values.lock().unwrap();
        let result: Option<&CacheElem> = map.get(&key);
        if let Some(ref value) = result  {
            let z: &CacheElem = value;
            let clone_value: CacheElem = z.clone();
            return Some(clone_value);
        }
        None
    }


    pub fn remove(&self, key: u64)  {
        self.values.lock().unwrap().remove(&key);
    }



}