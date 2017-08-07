extern crate grpc;
extern crate futures;



use std::env;
use std::mem;
use std::ptr;
use std::str;
use std::slice;
use std::{thread, time};
use std::sync::mpsc;
use std::time::Duration;
use std::collections::HashMap;
use service_grpc::MixerClient;
use report::ReportRequest;
use attributes::Attributes;
use attributes::StringMap;
use service_grpc::Mixer;

use protobuf::well_known_types::Timestamp;
use bindings::ngx_http_request_s;
use bindings::ngx_http_headers_in_t;
use bindings::ngx_http_headers_out_t;
use bindings::ngx_cycle_t;
use bindings::ngx_int_t;
use bindings::ngx_str_t;
use bindings::NGX_OK;
use nginx_http::list_iterator;
use nginx_http::log;

use attr_dict::AttributeWrapper;


const REQUEST_HEADER: &str = "request.headers";
const TARGET_SERVICE: &str = "target.service";
const REQUEST_HOST: &str = "request.host";
const REQUEST_METHOD: &str = "request.method";
const REQUEST_PATH: &str =  "request.path";
const REQUEST_REFER: &str = "request.referer";
const REQUEST_SCHEME: &str = "request.scheme";
const REQUEST_SIZE: &str = "request.size";
const REQUEST_TIME: &str = "request.time";
const REQUEST_USERAGENT: &str = "request.useragent";
const RESPONSE_CODE: &str = "response.code";
const RESPONSE_DURATION: &str = "response.duration";
const RESPONSE_SIZE: &str = "response.size";
const RESPONSE_HEADERS: &str = "response.headers";






/**
  filter nginx
 */
/*
#[no_mangle]
public extern fn ngx_int_t ngx_http_istio_mixer_filter(request: *const ngx_http_request_s) -> ngx_int_t {
{

    log(&format!("start invoking istio mixer filter");

    ngx_http_mixer_main_conf_t *conf = *rngx_http_istio_mixer_module);

    ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "using server: %*s",conf->mixer_server.len,conf->mixer_server.data);

    // invoke mix client
    mixer_client(r);

    ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "finish calling istio filter");

    return ngx_http_next_header_filter(r);

} */




#[repr(C)]
pub struct ngx_http_mixer_main_conf_t {
    mixer_server: ngx_str_t,
    mixer_port: ngx_int_t
}


#[no_mangle]
pub extern fn mixer_client(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t)  {

    let mut req = ReportRequest::new();
    let mut attr = AttributeWrapper::new();

    process_request_attribute(request, &mut attr);
    process_response_attribute(request, &mut attr);

    req.set_attribute_update(attr.attributes);

    send(main_config, req);

}


// init mixer
#[no_mangle]
pub extern fn mixer_init(cycle: &ngx_cycle_t) -> ngx_int_t {

    log(&format!("init mixer start "));
    thread::spawn(|| {
        let mut i = 0;
        loop {
            log(&format!("mixer thread sleeping: {}",i));
            let second = time::Duration::new(5,0);
            thread::sleep(second);
            log(&format!("mixer wake from sleep "));
            i = i + 1;
        }

    });
    log(&format!("init mixer end "));
    return NGX_OK as ngx_int_t;
}

#[no_mangle]
pub extern fn mixer_exit() {
    log(&format!("mixer exit "));
}


static mut req_index: i64 = 0;

fn send(main_config: &ngx_http_mixer_main_conf_t, mut req: ReportRequest)  {

    unsafe {
        req.set_request_index(req_index);
        req_index = req_index + 1;
    }

    let mut requests = Vec::new();
    requests.push(req);

    let server_name = main_config.mixer_server.to_str();
    let server_port = main_config.mixer_port as u16;

    log(&format!("server: {}, port {}",server_name, server_port));

    let  client = MixerClient::new_plain( server_name, server_port , Default::default()).expect("init");

    let resp = client.report(grpc::RequestOptions::new(), grpc::StreamingRequest::iter(requests));

    resp.wait_drop_metadata().count();

}





// process request attribute,
// loop over request header and add to dictionary
// then use that to build new string map
fn process_request_attribute(request: & ngx_http_request_s, attr: &mut AttributeWrapper, )  {

    let headers_in = request.headers_in;


    let host = headers_in.host_str();
    log(&format!("request host {}",host));
    attr.insert_string_attribute(REQUEST_HOST, host);

    let method = request.method_name.to_str();
    log(&format!("request method {}",method));
    attr.insert_string_attribute(REQUEST_METHOD, method);

    let path = request.uri.to_str();
    log(&format!("request path {}",path));
    attr.insert_string_attribute(REQUEST_PATH, path);

    let referer = headers_in.referer_str();
    if let Some(refererStr) = referer {
        log(&format!("request referrer {}",refererStr));
        attr.insert_string_attribute(REQUEST_REFER, refererStr);
    }

    let scheme = request.http_protocol.to_str();
    log(&format!("request scheme {}",scheme));
    attr.insert_string_attribute(REQUEST_SCHEME, "http"); // hard code now

    let request_size = request.request_length;
    log(&format!("request size {}",request_size));
    attr.insert_int64_attribute(REQUEST_SIZE, request_size);

    let mut request_time = Timestamp::new();
    request_time.set_seconds(request.start_sec);
    request_time.set_nanos(request.start_msec as i32);
    attr.insert_time_stamp_attribute(REQUEST_TIME, request_time);

    let user_agent = headers_in.user_agent_str();
    log(&format!("request user agent {}",user_agent));
    attr.insert_string_attribute(REQUEST_USERAGENT, user_agent);



    // fill in the string value
    let mut map: HashMap<i32,String> = HashMap::new();
    {
        for (name,value) in headers_in.headers_iterator()   {
            log(&format!("header name: {}, value: {}",&name,&value));

            let index  = attr.string_index(&name[..]);
            map.insert(index,value);
               //     log(&format!("match existing index: {}",index));
        }
    }

    attr.insert_string_map(REQUEST_HEADER, map);

}




fn process_response_attribute(request: &ngx_http_request_s, attr: &mut AttributeWrapper, )  {


    let headers_out =  request.headers_out;


    let response_code = headers_out.status;
    log(&format!("response code {}",response_code));
    attr.insert_int64_attribute(RESPONSE_CODE, response_code as i64);

    let content_length = headers_out.content_length_n;
    log(&format!("content length {}",content_length));
    attr.insert_int64_attribute(RESPONSE_SIZE, content_length);

    let duration = headers_out.date_time - request.start_sec;
    log(&format!("response duration {}",duration));
    attr.insert_int64_attribute(RESPONSE_DURATION, 5000);

    // fill in the string value
    let mut map: HashMap<i32,String> = HashMap::new();
    {
        for (name,value) in headers_out.headers_iterator()   {
            log(&format!("out header name: {}, value: {}",&name,&value));

            let index  = attr.string_index(&name[..]);
            map.insert(index,value);

        }
    }

    attr.insert_string_map(RESPONSE_HEADERS, map);

}
