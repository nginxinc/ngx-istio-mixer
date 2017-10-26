use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::clone::Clone;

use mixer::check::CheckResponse;
use mixer::check::ReferencedAttributes_Condition;
use ngx_rust::nginx_http::log;
use super::status::StatusCodeEnum;
use super::options::CheckOptions;
use super::status::Status;
use attribute::attr_wrapper::AttributeWrapper;
use super::lru_cache::LRUCache;
use super::referenced::Referenced;

/*
impl Hash for CheckResponse {
    fn hash<H: Hasher>(&self, state: &mut H) {

        let condition = self.get_precondition();
        let attributes = condition.get_attributes();
        let status = condition.get_status();
        let valid_duration = condition.get_valid_duration();
        let use_count = condition.get_valid_use_count();
        let ref_attr = condition.get_referenced_attributes();
        //   let quota  = response.get_quotas();
 
        log(&format!("check attributes :{:?} ",attributes));
         log(&format!("check ref attributes :{:?} ",ref_attr));
        log(&format!("success calling check status:{:?}, duration: {:?}",
        status,valid_duration));

        let words = ref_attr.get_words();
        let matches = ref_attr.get_attribute_matches();
        log(&format!(" ref attr words :{:?} ",words));
        log(&format!(" ref attr matches :{:?} ",matches));

        for condition in matches  {
           
            let name = condition.get_name();
            let condition = condition.get_condition();

             log(&format!("condition name :{:?} ",name));  
            
             match condition  {
                 ReferencedAttributes_Condition::CONDITION_UNSPECIFIED =>   {
                     log(&format!("unspecified"))
                 },
                 ReferencedAttributes_Condition::ABSENCE =>  {
                     log(&format!("absence"))
                 },
                 ReferencedAttributes_Condition::EXACT =>  {
                     log(&format!("exact"))
                 }
                 ReferencedAttributes_Condition::REGEX =>  {
                     log(&format!("regex"))
                 }
             }

        }
    }
}
*/

pub struct CheckCache {

    check_options: CheckOptions,
    cache: LRUCache,
    referenced_map: HashMap<String,Referenced>
}



impl CheckCache  {


    pub fn new() -> CheckCache {
        CheckCache {
            check_options: CheckOptions::new(),
            cache: LRUCache::new(),
            referenced_map: HashMap::new()
        }
    }


    pub fn check(&self, attributes: &AttributeWrapper, result: &CheckResult) {

    }

    // check attribute for time
    // Status CheckCache::Check(const Attributes& attributes, Tick time_now)
    fn check_attribute_time(&self, attributes: &AttributeWrapper, tick: SystemTime) -> Status {

        for( key,reference) in &self.referenced_map  {

            let some_signature = reference.signature(attributes,"");

            match some_signature {
                None => continue,
                Some(signature) =>  {
                    if let Some(value) = self.cache.get(signature) {
                        if value.is_expired(tick) {
                            self.cache.remove(signature);
                            return Status::with_code(StatusCodeEnum::NOT_FOUND)
                        }

                        return value.get_status();
                    }
                }

            }

        }

        return Status::with_code(StatusCodeEnum::NOT_FOUND)
    }


}


struct CheckResult {

    status: Status

}

impl CheckResult  {

    pub fn new() -> CheckResult  {
        CheckResult {
            status: Status::new()
        }

    }


    pub fn is_cache_hit(&self) -> bool {
        self.status.get_error_code() as i32 !=  StatusCodeEnum::UNAVAILABLE as i32
    }

    pub fn set_status( &mut self, status: Status) {
        self.status = status;
    }


        /*
        if(response.has_precondition()) {

            if(response.get_precondition().has_precondition()) {
                expire_time_ =
          time_now + ToMilliseonds(response.precondition().valid_duration());
            }
        }


        let condition = response.get_precondition();
        let attributes = condition.get_attributes();
        let status = condition.get_status();
        let valid_duration = condition.get_valid_duration();
        let use_count = condition.get_valid_use_count();
        let ref_attr = condition.get_referenced_attributes();
        //   let quota  = response.get_quotas();

        log(&format!("check attributes :{:?} ",attributes));
        log(&format!("check ref attributes :{:?} ",ref_attr));
        log(&format!("success calling check status:{:?}, duration: {:?}",
                     status,valid_duration));

        let words = ref_attr.get_words();
        let matches = ref_attr.get_attribute_matches();
        log(&format!(" ref attr words :{:?} ",words));
        log(&format!(" ref attr matches :{:?} ",matches));

        for condition in matches  {

            let name = condition.get_name();
            let condition = condition.get_condition();

            log(&format!("condition name :{:?} ",name));

            match condition  {
                ReferencedAttributes_Condition::CONDITION_UNSPECIFIED =>   {
                    log(&format!("unspecified"))
                },
                ReferencedAttributes_Condition::ABSENCE =>  {
                    log(&format!("absence"))
                },
                ReferencedAttributes_Condition::EXACT =>  {
                    log(&format!("exact"))
                }
                ReferencedAttributes_Condition::REGEX =>  {
                    log(&format!("regex"))
                }
            }

        }
        */



}




#[test]
fn test_cache_result_hit() {

    let cache_result = CheckResult::new();
    assert_eq!(cache_result.is_cache_hit(),true);
}
