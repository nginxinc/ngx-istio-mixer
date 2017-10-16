extern crate grpc;
extern crate futures;
extern crate ngx_rust;


use std::str;
use std::{thread };
use std::sync::mpsc::{channel,Sender,Receiver};
use std::sync::Mutex;
use std::collections::HashMap;

use mixer::service_grpc::MixerClient;
use mixer::report::ReportRequest;
use mixer::attributes::Attributes;
use mixer::service_grpc::Mixer;

use protobuf::well_known_types::Timestamp;
use protobuf::RepeatedField;
use ngx_rust::bindings::ngx_http_request_s;
use ngx_rust::bindings::ngx_int_t;
use ngx_rust::bindings::NGX_OK;
use ngx_rust::nginx_http::log;


use ngx::mixer_location::ngx_http_mixer_main_conf_t;



#[no_mangle]
pub extern fn nginmesh_mixer_check_handler(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t)  -> ngx_int_t {

    log(&format!("rust mixer function called "));
    return NGX_OK as ngx_int_t;
}