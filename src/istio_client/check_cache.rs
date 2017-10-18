
use mixer::check::CheckResponse;
use ngx_rust::nginx_http::log;

pub struct CheckCache {


}


impl CheckCache  {

    pub fn new() -> CheckCache  {
        CheckCache {}

    }


    pub fn set_reponse( &self, response: &CheckResponse )   {


          let condition = response.get_precondition();
            let attributes = condition.get_attributes();
        let status = condition.get_status();
        let valid_duration = condition.get_valid_duration();
        let use_count = condition.get_valid_use_count();
        let ref_attr = condition.get_referenced_attributes();
        //   let quota  = response.get_quotas();
 
        log(&format!("success calling check status:{:?}, attr:{:?}, duration: {:?}, ref_attr: {:?} ",
        status, attributes,valid_duration,ref_attr));


    }


}