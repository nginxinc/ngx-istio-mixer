extern crate grpc;
extern crate futures;
extern crate ngx_rust;


use std::str;
use std::sync::mpsc::{channel};
use std::sync::Mutex;
use std::collections::HashMap;

use mixer::service_grpc::MixerClient;
use mixer::report::ReportRequest;
use mixer::attributes::Attributes;
use mixer::service_grpc::Mixer;


use protobuf::RepeatedField;
use ngx_rust::bindings::ngx_http_request_s;
use ngx_rust::nginx_http::log;

use ngx::attr_wrapper::AttributeWrapper;
use ngx::global_dict::GlobalDictionary;
use ngx::message_dict::MessageDictionary;

use ngx::mixer_location::ngx_http_mixer_main_conf_t;

use ngx::message::Channels;
use ngx::message::MixerInfo;
use ngx::request::process_request_attribute;


use ngx::global_dict::TARGET_SERVICE;

use ngx::global_dict::RESPONSE_CODE;
use ngx::global_dict::RESPONSE_DURATION;
use ngx::global_dict::RESPONSE_SIZE;
use ngx::global_dict::RESPONSE_HEADERS;
use ngx::global_dict::TARGET_IP;
use ngx::global_dict::TARGET_UID;




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

// background activy for report.
// receives report attributes and send out to mixer
pub fn mixer_report_background()  {

    let rx = CHANNELS.rx.lock().unwrap();

    loop {
        log(&format!("mixer report  thread waiting"));
        let info = rx.recv().unwrap();
        log(&format!("mixer report thread woke up"));

        let client = MixerClient::new_plain( &info.server_name, info.server_port , Default::default()).expect("init");

        let mut req = ReportRequest::new();
        let mut rf = RepeatedField::default();
        rf.push(info.attributes);
        req.set_attributes(rf);

        let resp = client.report(grpc::RequestOptions::new(), req);

        let result = resp.wait();

        log(&format!("mixer report thread: finished sending to mixer"));
    }
}


// send to background thread using channels
fn send_dispatcher(main_config: &ngx_http_mixer_main_conf_t, attr: Attributes)  {

    let server_name = main_config.mixer_server.to_str();
    let server_port = main_config.mixer_port as u16;

    let tx = CHANNELS.tx.lock().unwrap().clone();
    let info = MixerInfo { server_name: String::from(server_name), server_port: server_port, attributes: attr};
    tx.send(info.clone());


   // log(&format!("server: {}, port {}",server_name, server_port));

    log(&format!("send attribute to mixer report background task"));

}



#[no_mangle]
pub extern fn nginmesh_mixer_report_handler(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t)  {


    let mut attr = AttributeWrapper::new();

    process_istio_attr(main_config,&mut attr);
    process_request_attribute(request, &mut attr);
    process_response_attribute(request, &mut attr);


    let mut message_dict = MessageDictionary::new(GlobalDictionary::new());
    send_dispatcher(main_config, attr.as_attributes(&mut message_dict));

}


/*
 * Istio attributes such as source.ip are passed as http header and also send out source headewr
 */
fn process_istio_attr(main_config: &ngx_http_mixer_main_conf_t, attr: &mut AttributeWrapper) {

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

    let target_service = main_config.target_service.to_str();
    if target_service.len() > 0 {
        log(&format!("target service founded!"));
        attr.insert_string_attribute(TARGET_SERVICE,target_service);
    }

}





fn process_response_attribute(request: &ngx_http_request_s, attr: &mut AttributeWrapper, )  {

    let headers_out =  request.headers_out;

    attr.insert_int64_attribute(RESPONSE_CODE, headers_out.status as i64);
    attr.insert_int64_attribute(RESPONSE_SIZE, headers_out.content_length_n);

    let duration = headers_out.date_time - request.start_sec;
    attr.insert_int64_attribute(RESPONSE_DURATION, 5000);

    // fill in the string value
    let mut map: HashMap<String,String> = HashMap::new();
    {
        for (name,value) in headers_out.headers_iterator()   {
            log(&format!("processing out header name: {}, value: {}",&name,&value));

            map.insert(name,value);

        }
    }

    attr.insert_string_map(RESPONSE_HEADERS, map);

}
