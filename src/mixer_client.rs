extern crate grpc;
extern crate futures;
extern crate libc;


use std::env;
use std::mem;
use std::ptr;
use std::str;
use std::slice;
use std::ffi::CString;
use std::collections::HashMap;
use service_grpc::MixerClient;
use report::ReportRequest;
use attributes::Attributes;
use service_grpc::Mixer;
use bindings::ngx_http_request_s;
use bindings::ngx_http_request_t;
use bindings::ngx_list_part_t;
use bindings::ngx_table_elt_t;
use bindings::ngx_uint_t;
use bindings::ngx_str_t;
use bindings::ngx_log_error_core;
use bindings::NGX_LOG_ERR;
use bindings::ngx_cycle;


static REQUEST_HEADER: i32 = 0;
static TARGET_SERVICE: i32 = 1;

impl ngx_str_t  {
    // convert nginx string to str slice
    fn to_str(&self) -> &str  {

        unsafe {
            let slice = slice::from_raw_parts(self.data,self.len) ;
            return str::from_utf8(slice).unwrap();
        }            
   
    }
}


// extract request.header from nginx request
// 
fn extract_request_header_from_nginx(ngxRequest: *const ngx_http_request_s)  -> String {

    let mut outHeader = String::from("");

      // extract 
    unsafe  {
        let mut part: *const ngx_list_part_t  = &(*ngxRequest).headers_in.headers.part ;
        let mut h: *const ngx_table_elt_t =  (*part).elts as *const ngx_table_elt_t;

        let mut i: ngx_uint_t = 0;
        let mut done = false;

        while !done  {
            if i >= (*part).nelts  {
                if (*part).next.is_null() {
                    done = true;
                    break;
                }

                part = (*part).next;
                h = (*part).elts as *mut ngx_table_elt_t;
                i = 0;
            }

            let header: *const ngx_table_elt_t = h.offset(i as isize);

            let header_name: ngx_str_t = (*header).key;   
            ngx_log_error_core(NGX_LOG_ERR as usize, (*ngx_cycle).log, 0, CString::new("request header: %*s").unwrap().as_ptr(),
                header_name.len,header_name.data);         
            outHeader.push_str(header_name.to_str());
           
           
            outHeader.push_str(":");

            let header_value: ngx_str_t = (*header).value;
            ngx_log_error_core(NGX_LOG_ERR as usize, (*ngx_cycle).log, 0, CString::new("request value: %*s").unwrap().as_ptr(),
                header_value.len,header_value.data);  

            outHeader.push_str(header_value.to_str());
  
                  
            i = i + 1;

        }
    }

    return outHeader;

}


#[no_mangle]
pub extern fn mixer_client(ngxRequest: *const ngx_http_request_s) -> *const u8 {


    let client = MixerClient::new_plain("localhost", 9091, Default::default()).expect("init");

    let mut requests = Vec::new();
    let mut req = ReportRequest::new();
    let mut attr = Attributes::new();
    //attr.set_string_attributes("")
    req.set_request_index(0);

    // set up attribute dictionary 
    let mut dictValues: HashMap<i32,String> = HashMap::new();
    dictValues.insert(REQUEST_HEADER,String::from("request.headers"));
    dictValues.insert(TARGET_SERVICE,String::from("target.service"));
    attr.set_dictionary(dictValues);


    let mut stringValues: HashMap<i32,String> = HashMap::new();
 
    let outHeader = extract_request_header_from_nginx(ngxRequest);
  

    stringValues.insert(REQUEST_HEADER,outHeader);

    
    attr.set_string_attributes(stringValues);

    req.set_attribute_update(attr);


    requests.push(req);


    let resp = client.report(grpc::RequestOptions::new(), grpc::StreamingRequest::iter(requests));

    resp.wait_drop_metadata().count();

    "Hello, world!\0".as_ptr()
}


