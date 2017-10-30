#![feature(conservative_impl_trait)]

use std::time::{ SystemTime};
use std::collections::HashMap;
use std::clone::Clone;

use mixer::check::CheckResponse;
use transport::status::{ Status, StatusCodeEnum} ;
use super::options::CheckOptions;
use attribute::attr_wrapper::AttributeWrapper;
use super::lru_cache::LRUCache;
use super::referenced::Referenced;


#[allow(dead_code)]
fn on_response(cache: &CheckCache,status: Status,_result: &mut CheckResult, attributes: &AttributeWrapper, response: &CheckResponse) -> Status {

    if !status.ok() {
        if cache.options.network_fail_open {
            return Status::new();
        }  else {
            return status;
        }
    } else {
        return cache.cache_response(attributes,response,SystemTime::now());
    }

}

#[allow(dead_code)]
pub struct CheckCache {

    options: CheckOptions,
    cache: LRUCache,
    referenced_map: HashMap<String,Referenced>
}


#[allow(dead_code)]
impl CheckCache  {


    pub fn new() -> CheckCache {
        CheckCache {
            options: CheckOptions::new(),
            cache: LRUCache::new(),
            referenced_map: HashMap::new()
        }
    }


    pub fn check(&self, attributes: &AttributeWrapper) -> CheckResult{


        let mut result =  CheckResult::new(on_response);
        let status = self.check_attribute_time(attributes, SystemTime::now());
        if status.get_error_code() != StatusCodeEnum::NOT_FOUND {
            result.set_status(status);
        }

        result

    }


    fn cache_response(&self, _attributes: &AttributeWrapper, _response: &CheckResponse, _time_now: SystemTime) -> Status {
        return Status::new()
    }

    // check attribute for time
    // Status CheckCache::Check(const Attributes& attributes, Tick time_now)
    fn check_attribute_time(&self, attributes: &AttributeWrapper, tick: SystemTime) -> Status {


        for( _key,reference) in &self.referenced_map  {

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


#[allow(dead_code)]
pub struct CheckResult {

    status: Status,
    on_response: fn(cache: &CheckCache,status: Status,result: &mut CheckResult, attributes: &AttributeWrapper, response: &CheckResponse) -> Status

}

#[allow(dead_code)]
impl CheckResult {

    pub fn new( on_response: fn(cache: &CheckCache,status: Status,result: &mut CheckResult,attributes: &AttributeWrapper, response: &CheckResponse) -> Status) -> CheckResult {
        CheckResult {
            status: Status::new(),
            on_response
        }
    }


    pub fn is_cache_hit(&self) -> bool {
        self.status.get_error_code() as i32 !=  StatusCodeEnum::UNAVAILABLE as i32
    }

    pub fn get_status(&self) -> Status  {
        self.status.clone()
    }

    pub fn set_status( &mut self, status: Status) {
        self.status = status;
    }



    pub fn set_response(&mut self, cache: &CheckCache,status: Status,attributes: &AttributeWrapper, response: &CheckResponse) {
        let handler = self.on_response;
        let status = handler(cache, status,self,attributes,response);
        self.set_status(status);
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

#[allow(dead_code)]
fn test_response1(_cache: &CheckCache,_status: Status,_result: &mut CheckResult, _attributes: &AttributeWrapper, _response: &CheckResponse) -> Status {
    Status::new()
}


#[test]
fn test_check_result_cache_hit() {


    let cache_result = CheckResult::new(test_response1);
    assert_eq!(cache_result.is_cache_hit(),true);
}

#[allow(dead_code)]
fn test_response2(_cache: &CheckCache,_status: Status,_result: &mut CheckResult, _attributes: &AttributeWrapper, _response: &CheckResponse) -> Status {
    _status
}

#[test]
fn test_check_result_set_response()  {


    let mut check_result = CheckResult::new(test_response2);
    check_result.set_response( &CheckCache::new(),Status::with_code(StatusCodeEnum::CANCELLED), &AttributeWrapper::new(),&CheckResponse::new());

    assert_eq!(check_result.get_status().get_error_code(), StatusCodeEnum::CANCELLED,"status should be cancelled");
}
