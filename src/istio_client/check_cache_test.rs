

use super::check_cache::CheckCache;


#[test]
fn test_cache_ok() {

    let cache = CheckCache::new();
     assert_eq!(cache.is_cache_hit(),true);
}

