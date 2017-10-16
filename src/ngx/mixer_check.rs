extern crate grpc;
extern crate futures;
extern crate ngx_rust;


use std::str;
use std::{thread };
use std::sync::mpsc::{channel,Sender,Receiver};
use std::sync::Mutex;
use std::collections::HashMap;

use mixer::service_grpc::MixerClient;
use mixer::attributes::Attributes;
use mixer::service_grpc::Mixer;
use mixer::check::CheckRequest;

use protobuf::well_known_types::Timestamp;
use protobuf::RepeatedField;
use ngx_rust::bindings::ngx_http_request_s;
use ngx_rust::bindings::ngx_int_t;
use ngx_rust::bindings::NGX_OK;
use ngx_rust::nginx_http::log;


use ngx::mixer_location::ngx_http_mixer_main_conf_t;

use ngx::attr_wrapper::AttributeWrapper;
use ngx::global_dict::GlobalDictionary;
use ngx::message_dict::MessageDictionary;

use ngx::global_dict::REQUEST_HEADER;
use ngx::global_dict::TARGET_SERVICE;
use ngx::global_dict::REQUEST_HOST;
use ngx::global_dict::REQUEST_METHOD;
use ngx::global_dict::REQUEST_PATH;
use ngx::global_dict::REQUEST_REFER;
use ngx::global_dict::REQUEST_SCHEME;
use ngx::global_dict::REQUEST_SIZE;
use ngx::global_dict::REQUEST_TIME;
use ngx::global_dict::REQUEST_USERAGENT;
use ngx::global_dict::RESPONSE_CODE;
use ngx::global_dict::RESPONSE_DURATION;
use ngx::global_dict::RESPONSE_SIZE;
use ngx::global_dict::RESPONSE_HEADERS;
use ngx::global_dict::SOURCE_IP;
use ngx::global_dict::SOURCE_UID;
use ngx::global_dict::TARGET_IP;
use ngx::global_dict::TARGET_UID;


#[no_mangle]
pub extern fn nginmesh_mixer_check_handler(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t)  -> ngx_int_t {

    log(&format!("rust mixer function called "));


    let mut attr_wrapper = AttributeWrapper::new();
    process_request_attribute(request, &mut attr_wrapper);

    let mut message_dict = MessageDictionary::new(GlobalDictionary::new());
    let attributes = attr_wrapper.as_attributes(&mut message_dict);

    let server_name = main_config.mixer_server.to_str();
    let server_port = main_config.mixer_port as u16;

    let client = MixerClient::new_plain( &server_name, server_port , Default::default()).expect("init");

    let mut check_request = CheckRequest::new();
    check_request.set_attributes(attributes);

    let resp = client.check(grpc::RequestOptions::new(), check_request);

    let result = resp.wait();

     log(&format!("response fro mixer {:?}",result));
    return NGX_OK as ngx_int_t;
}

// process request attribute,
// loop over request header and add to dictionary
// then use that to build new string map
fn process_request_attribute(request: & ngx_http_request_s, attr: &mut AttributeWrapper )  {

    let headers_in = request.headers_in;


    // fill in the string value
    let mut map: HashMap<String,String> = HashMap::new();
    {
        for (name,value) in headers_in.headers_iterator()   {
            log(&format!("in header name: {}, value: {}",&name,&value));

            map.insert(name,value);
        }
         
    }

    attr.insert_string_map(REQUEST_HEADER, map);

}