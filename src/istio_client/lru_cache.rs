use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard };
use super::status::Status;
use std::time::{Duration, SystemTime};

// simple LRU Cache, index by hash

pub struct CacheElem  {
    status: Status,
    expire_time: SystemTime,
    use_count: u32,
}


impl Clone for CacheElem {

    fn clone(&self) -> Self {
        CacheElem {
            status: self.status.clone(),
            expire_time: self.expire_time.clone(),
            use_count: self.use_count.clone()
        }
    }
}


impl  CacheElem  {

    pub fn new() -> CacheElem  {
        CacheElem {
            status: Status::new(),
            expire_time: SystemTime::now() + Duration::new(3200*24, 0),
            use_count: 0
        }
    }


    // check if value is expired
    pub fn is_expired(&self, time: SystemTime) -> bool  {
        self.expire_time > time
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

}


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