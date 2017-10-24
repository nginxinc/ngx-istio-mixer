use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, SystemTime};

use mixer::check::CheckResponse;
use mixer::check::ReferencedAttributes_Condition;
use ngx_rust::nginx_http::log;
use super::code::StatusCodeEnum;
use super::options::CheckOptions;

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

    status: StatusCodeEnum,
    check_options: CheckOptions
}


impl CheckCache  {

    pub fn new() -> CheckCache  {
        CheckCache {
            status: StatusCodeEnum::OK,
            check_options: CheckOptions::new()
        }

    }

    pub fn is_cache_hit(&self) -> bool {
        self.status as i32 !=  StatusCodeEnum::UNAVAILABLE as i32
    }

    pub fn set_reponse( &self, response: &CheckResponse )   {

        /*
        if(response.has_precondition()) {

            if(response.get_precondition().has_precondition()) {
                expire_time_ =
          time_now + ToMilliseonds(response.precondition().valid_duration());
            }
        }
        */

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

    }

}



struct CacheElem<'a>  {
    status: StatusCodeEnum,
    expire_time: SystemTime,
    use_count: u32,
    parent: &'a CheckCache
}

impl<'a>  CacheElem<'a>  {

    pub fn new(cache: &CheckCache) -> CacheElem  {
        CacheElem {
            status: StatusCodeEnum::OK,
            expire_time: SystemTime::now() + Duration::new(3200*24, 0),
            use_count: 0,
            parent: cache
        }
    }



    fn is_expired(&self) -> bool  {
        false
    }

    fn status(&self) -> StatusCodeEnum {
        self.status
    }


}