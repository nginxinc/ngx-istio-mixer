// This is from ISTIO mixerclient/include/options.h

// Options controlling check behavior.
pub struct CheckOptions {
 
  // Maximum number of cache entries kept in the cache.
  // Set to 0 will disable caching.
   pub num_entries: u32,

  // If true, Check is passed for any network failures.
   pub network_fail_open: bool
}

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





