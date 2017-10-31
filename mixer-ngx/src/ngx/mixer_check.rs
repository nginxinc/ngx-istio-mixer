
extern crate grpc;
extern crate futures;
extern crate ngx_mixer_transport;

use ngx_rust::bindings::ngx_http_request_s;
use ngx_rust::bindings::ngx_int_t;
use ngx_rust::bindings::{ NGX_OK, NGX_DECLINED };
use ngx_rust::nginx_http::log;

use super::mixer_location::ngx_http_mixer_main_conf_t;


use super::request::process_request_attribute;

use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::attribute::global_dict::TARGET_SERVICE;
use ngx_mixer_transport::attribute::global_dict::TARGET_IP;
use ngx_mixer_transport::attribute::global_dict::TARGET_UID;


use ngx_mixer_transport::istio_client::mixer_client_wrapper::MixerClientWrapper ;
use ngx_mixer_transport::transport::mixer_grpc::GrpcTransport;
use ngx_mixer_transport::transport::server_info::MixerInfo;
use futures::future::Future;



lazy_static! {
    static ref DEFAULT_MIXER_CLIENT: MixerClientWrapper = MixerClientWrapper::new();
}




// perform check
pub fn check(server_name: &str,server_port: u16, attr: AttributeWrapper) -> bool {


    let info = MixerInfo { server_name: String::from(server_name), server_port: server_port};

    let transport = GrpcTransport::new(info,attr);
    let result = DEFAULT_MIXER_CLIENT.check(transport).wait();

   // log(&format!("send attribute to mixer check background task, {:?}",result));

    true

}


#[no_mangle]
pub extern fn nginmesh_mixer_check_handler(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t)  -> ngx_int_t {

    log(&format!("rust mixer function called "));


    let mut attributes = AttributeWrapper::new();
    process_istio_attr(main_config,&mut attributes);
    process_request_attribute(request, &mut attributes);

    let server_name = main_config.mixer_server.to_str();
    let server_port = main_config.mixer_port as u16;


    if !check(server_name,server_port, attributes) {
        return NGX_DECLINED as ngx_int_t;
    }

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