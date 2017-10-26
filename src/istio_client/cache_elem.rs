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

    pub fn with_status_expired(status: Status, expire_time: SystemTime) -> CacheElem {
        CacheElem {
            status,
            expire_time,
            use_count: 0
        }
    }


    // check if value is expired
    pub fn is_expired(&self, time: SystemTime) -> bool  {
        time >= self.expire_time
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

}


#[test]
fn test_is_expired() {

    let elem = CacheElem::new();
    assert!(!elem.is_expired(SystemTime::now()),"should not expired");

    let elem = CacheElem::with_status_expired(Status::new(),SystemTime::now() + Duration::new(1000,0));
    assert!(!elem.is_expired(SystemTime::now()),"should not expired");
}




