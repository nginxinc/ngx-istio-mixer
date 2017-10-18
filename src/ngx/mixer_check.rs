extern crate grpc;
extern crate futures;
extern crate ngx_rust;

use std::sync::mpsc::{channel};
use std::sync::Mutex;

use mixer::service_grpc::MixerClient;
use mixer::attributes::Attributes;
use mixer::service_grpc::Mixer;
use mixer::check::CheckRequest;

use ngx_rust::bindings::ngx_http_request_s;
use ngx_rust::bindings::ngx_int_t;
use ngx_rust::bindings::NGX_OK;
use ngx_rust::nginx_http::log;


use ngx::mixer_location::ngx_http_mixer_main_conf_t;

use ngx::attr_wrapper::AttributeWrapper;
use ngx::global_dict::GlobalDictionary;
use ngx::message_dict::MessageDictionary;
use ngx::message::Channels;
use ngx::message::MixerInfo;
use ngx::request::process_request_attribute;



use ngx::global_dict::TARGET_SERVICE;
use ngx::global_dict::TARGET_IP;
use ngx::global_dict::TARGET_UID;


use istio_client::check_cache::CheckCache;


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


// process check event in the background
pub fn mixer_check_background()  {

    let rx = CHANNELS.rx.lock().unwrap();
    let cache = CheckCache::new();

    loop {
        log(&format!("mixer check thread waiting"));
        let info = rx.recv().unwrap();
        log(&format!("mixer check thread woke up"));

        let client = MixerClient::new_plain( &info.server_name, info.server_port , Default::default()).expect("init");

        let mut check_request = CheckRequest::new();
        check_request.set_attributes(info.attributes);

        let result = client.check(grpc::RequestOptions::new(), check_request).wait();

     //       log(&format!("mixer check {:?}",result));
        match result   {
            Ok(response) =>  {
                let (m1, check_response, m2) = response;
                cache.set_reponse(&check_response);
            },

            Err(err)  =>  {
                 // TODO: fix log error to nginx error logger
                 log(&format!("error calling check {:?}",err));
            }

        }

        
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

    log(&format!("send attribute to mixer check background task"));

}


#[no_mangle]
pub extern fn nginmesh_mixer_check_handler(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t)  -> ngx_int_t {

    log(&format!("rust mixer function called "));


    let mut attr_wrapper = AttributeWrapper::new();
    process_istio_attr(main_config,&mut attr_wrapper);
    process_request_attribute(request, &mut attr_wrapper);

    let mut message_dict = MessageDictionary::new(GlobalDictionary::new());
    let attributes = attr_wrapper.as_attributes(&mut message_dict);
    send_dispatcher(main_config, attributes);

    return NGX_OK as ngx_int_t;
}



/*
 * Istio attributes such as source.ip are passed as http header and also send out source headewr
 * TODO: this is duplicate from mixer_reports. consolidate into common util
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