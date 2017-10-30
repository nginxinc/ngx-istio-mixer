// This is from ISTIO mixerclient/include/options.h

// Options controlling check behavior.
pub struct CheckOptions {
 
  // Maximum number of cache entries kept in the cache.
  // Set to 0 will disable caching.
   pub num_entries: u32,

  // If true, Check is passed for any network failures.
   pub network_fail_open: bool
}

#[allow(dead_code)]
impl CheckOptions  {

  pub fn new() -> CheckOptions  {
      CheckOptions  {
        num_entries: 10000,
        network_fail_open: true
      }
  }

  pub fn num_entries(cache_entries: u32) -> CheckOptions {
      CheckOptions  {
        num_entries: cache_entries,
        network_fail_open: true
      }
  }

}


// Options controlling report batch.
pub struct ReportOptions {

 // Maximum number of reports to be batched.
  pub max_batch_entries: u32,

  // Maximum milliseconds a report item stayed in the buffer for batching.
  pub max_batch_time_ms: u32
}

impl ReportOptions {

  // // Default to batch up to 1000 reports or 1 seconds.
  pub fn new() -> ReportOptions  {
      ReportOptions  {
        max_batch_entries: 1000,
        max_batch_time_ms: 1000
      }
  }

}


// Options controlling quota behavior.
pub struct QuotaOptions {
 

 // Maximum number of cache entries kept in the cache.
  // Set to 0 will disable caching.
  pub num_entries: u32,

  // Maximum milliseconds before an idle cached quota should be deleted.
  pub expiration_ms: u32
}

impl QuotaOptions  {

  pub fn new() -> QuotaOptions  {
      QuotaOptions  {
        num_entries: 10000,
        expiration_ms: 600000
      }
  }
}





