extern crate grpc;
extern crate futures;



use std::env;
use std::mem;
use std::ptr;
use std::str;
use std::slice;
use std::{thread, time};
use std::sync::mpsc::{channel,Sender,Receiver};
use std::sync::Mutex;
use std::time::Duration;
use std::collections::HashMap;

use time::get_time;


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
use bindings::ngx_flag_t;
use nginx_http::list_iterator;
use nginx_http::log;

use attr_dict::AttributeWrapper;
use encode:: { encode_istio_header, decode_istio_header };


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
const SOURCE_IP: &str = "source.ip";
const SOURCE_UID: &str = "source.uid";
const TARGET_IP: &str = "target.ip";
const TARGET_UID: &str = "target.uid";






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
    mixer_port: ngx_int_t,
    target_ip: ngx_str_t,
    target_uid: ngx_str_t

}

#[repr(C)]
pub struct ngx_http_mixer_loc_conf_t {
    enable: ngx_flag_t,              // for every location, we need flag to enable/disable mixer
}




// init mixer
#[no_mangle]
pub extern fn mixer_init(cycle: &ngx_cycle_t) -> ngx_int_t {

    log(&format!("init mixer start "));
    thread::spawn(|| {
        mixer_background();
    });
    log(&format!("init mixer end "));
    return NGX_OK as ngx_int_t;
}

#[no_mangle]
pub extern fn mixer_exit() {
    log(&format!("mixer exit "));
}


struct Channels<T> {
    pub tx: Mutex<Sender<T>>,
    pub rx: Mutex<Receiver<T>>
}


#[derive(Clone, Debug)]
struct MixerInfo  {
    server_name: String,
    server_port: u16,
    attributes: Attributes
}

// initialize channel that can be shared
lazy_static! {
    static ref CHANNELS: Channels<MixerInfo> = {
        let (tx, rx) = channel();

        Channels {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
        }
    };
}

// background actity handle mixer connection
fn mixer_background()  {

    let mut req_index: i64 = 0;
    let rx = CHANNELS.rx.lock().unwrap();

    loop {
        log(&format!("mixer send thread waiting: {}",req_index));
        let mut info = rx.recv().unwrap();
        log(&format!("mixer send thread woke up"));

        let client = MixerClient::new_plain( &info.server_name, info.server_port , Default::default()).expect("init");

        let mut requests = Vec::new();
        let mut req = ReportRequest::new();
        req.set_request_index(0);
        req.set_attribute_update(info.attributes);
        requests.push(req);

        let resp = client.report(grpc::RequestOptions::new(), grpc::StreamingRequest::iter(requests));

        resp.wait_drop_metadata().count();

        log(&format!("finished sending to mixer"));

        req_index = req_index + 1;
    }
}

/*
fn fillInAttributes(attr: &mut AttributeWrapper) {

    attr.insert_string_attribute(REQUEST_HOST,"35.202.158.195");
    attr.insert_string_attribute( TARGET_IP,"10.40.7.6");
    attr.insert_string_attribute(REQUEST_METHOD,"GET");
    attr.insert_string_attribute(TARGET_UID,"kubernetes://productpage-v1-3990756607-plqt5.default");


    let mut request_time = Timestamp::new();
    let current_time = get_time();
    request_time.set_seconds(current_time.sec);
    request_time.set_nanos(current_time.nsec as i32);
    attr.insert_time_stamp_attribute(REQUEST_TIME, request_time);



    // fill in the string value
    let mut map: HashMap<i32,String> = HashMap::new();
    let index  = attr.string_index("content-length");
    map.insert(index,String::from("10"));
    attr.insert_string_map(REQUEST_HEADER, map);

}
*/

fn send(main_config: &ngx_http_mixer_main_conf_t, attr: Attributes)  {

    let server_name = main_config.mixer_server.to_str();
    let server_port = main_config.mixer_port as u16;

    let tx = CHANNELS.tx.lock().unwrap().clone();
    let info = MixerInfo { server_name: String::from(server_name), server_port: server_port, attributes: attr};
    tx.send(info.clone());


   // log(&format!("server: {}, port {}",server_name, server_port));

    log(&format!("send attribute to mixer delegate"));

}



#[no_mangle]
pub extern fn mixer_client(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t)  {


    let mut attr = AttributeWrapper::new();

    process_istio_attr(request,main_config,&mut attr);
    process_request_attribute(request, &mut attr);
    process_response_attribute(request, &mut attr);

    send(main_config, attr.attributes);

}


/*
 * Istio attributes such as source.ip are passed as http header and also send out source headewr
 */
fn process_istio_attr(request: & ngx_http_request_s, main_config: &ngx_http_mixer_main_conf_t, attr: &mut AttributeWrapper) {

    // fill in target attributes
    let target_ip = main_config.target_ip.to_str();
    if target_ip.len() > 0 {
        log(&format!("target ip founded!"));
        attr.insert_string_attribute( TARGET_IP,target_ip);
    }

    let target_uid = main_config.target_uid.to_str();
    if target_uid.len() > 0 {
        log(&format!("target uid founded!"));
        attr.insert_string_attribute(TARGET_UID,target_uid);
    }


}


const SRC_IP_HEADER: &str = "X-ISTIO-SRC-IP";
const SRC_UID_HEADER: &str = "X-ISTIO-SRC-UID";


// process request attribute,
// loop over request header and add to dictionary
// then use that to build new string map
fn process_request_attribute(request: & ngx_http_request_s, attr: &mut AttributeWrapper )  {

    let headers_in = request.headers_in;


    attr.insert_string_attribute(REQUEST_HOST,  headers_in.host_str());
    attr.insert_string_attribute(REQUEST_METHOD, request.method_name.to_str());
    attr.insert_string_attribute(REQUEST_PATH, request.uri.to_str());

    let referer = headers_in.referer_str();
    if let Some(refererStr) = referer {
        attr.insert_string_attribute(REQUEST_REFER, refererStr);
    }

    let scheme = request.http_protocol.to_str();
    attr.insert_string_attribute(REQUEST_SCHEME, "http"); // hard code now


    attr.insert_int64_attribute(REQUEST_SIZE, request.request_length);

    let mut request_time = Timestamp::new();
    request_time.set_seconds(request.start_sec);
    request_time.set_nanos(request.start_msec as i32);
    attr.insert_time_stamp_attribute(REQUEST_TIME, request_time);

    attr.insert_string_attribute(REQUEST_USERAGENT, headers_in.user_agent_str());


    // fill in the string value
    let mut map: HashMap<i32,String> = HashMap::new();
    {
        for (name,value) in headers_in.headers_iterator()   {
            log(&format!("in header name: {}, value: {}",&name,&value));

            // TODO: remove header
            match name.as_ref()  {

                SRC_IP_HEADER  => {
                    log(&format!("source IP received {}",&value));
                    attr.insert_string_attribute( SOURCE_IP,&value);
                },

                SRC_UID_HEADER => {
                    log(&format!("source UID received {}",&value));
                    attr.insert_string_attribute( SOURCE_UID,&value);
                },
                _ => {
                    let index  = attr.string_index(&name[..]);
                    map.insert(index,value);
                }
            }


        }
    }

    attr.insert_string_map(REQUEST_HEADER, map);

}



fn process_response_attribute(request: &ngx_http_request_s, attr: &mut AttributeWrapper, )  {

    let headers_out =  request.headers_out;

    attr.insert_int64_attribute(RESPONSE_CODE, headers_out.status as i64);
    attr.insert_int64_attribute(RESPONSE_SIZE, headers_out.content_length_n);

    let duration = headers_out.date_time - request.start_sec;
    attr.insert_int64_attribute(RESPONSE_DURATION, 5000);

    // fill in the string value
    let mut map: HashMap<i32,String> = HashMap::new();
    {
        for (name,value) in headers_out.headers_iterator()   {
            log(&format!("processing out header name: {}, value: {}",&name,&value));

            let index  = attr.string_index(&name[..]);
            map.insert(index,value);

        }
    }

    attr.insert_string_map(RESPONSE_HEADERS, map);

}
