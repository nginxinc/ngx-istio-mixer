extern crate grpc;
extern crate futures;



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
use nginx_http::extract_request_header_from_nginx;


static REQUEST_HEADER: i32 = 0;
static TARGET_SERVICE: i32 = 1;


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


