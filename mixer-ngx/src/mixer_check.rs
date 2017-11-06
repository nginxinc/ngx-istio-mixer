
extern crate grpc;
extern crate futures;
extern crate ngx_mixer_transport;
extern crate ngx_rust;

use futures::future::Future;


use ngx_rust::bindings::ngx_http_request_s;
use ngx_rust::bindings::ngx_int_t;
use ngx_rust::bindings::{ NGX_OK, NGX_DECLINED };


use super::location_config:: ngx_http_mixer_loc_conf_t ;
use super::main_config::ngx_http_mixer_main_conf_t;


use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::transport::status:: { StatusCodeEnum };

use ngx_mixer_transport::istio_client::mixer_client_wrapper::MixerClientWrapper ;
use ngx_mixer_transport::transport::mixer_grpc::GrpcTransport;
use ngx_mixer_transport::transport::server_info::MixerInfo;

use super::config::MixerConfig;


lazy_static! {
    static ref DEFAULT_MIXER_CLIENT: MixerClientWrapper = MixerClientWrapper::new();
}




// perform check
pub fn check(server_name: &str,server_port: u16, attr: AttributeWrapper) -> bool {


    let info = MixerInfo { server_name: String::from(server_name), server_port: server_port};

    let transport = GrpcTransport::new(info,attr);
    let result = DEFAULT_MIXER_CLIENT.check(transport).wait();

    match result {
        Ok(_) => return true,
        Err(error) => {
            ngx_log!("rust check transport failed: {:?}", error);

            if error.get_error_code() == StatusCodeEnum::PERMISSION_DENIED {
                return false;
            }
        }
    }

    true

}


#[no_mangle]
pub extern fn nginmesh_mixer_check_handler(request: &ngx_http_request_s,
                               main_config: &ngx_http_mixer_main_conf_t,
                               loc_config: &ngx_http_mixer_loc_conf_t)  -> ngx_int_t {

    ngx_log!("rust mixer check handler called");

    let mut attributes = AttributeWrapper::new();
    main_config.process_istio_attr(&mut attributes);
    loc_config.process_istio_attr(&mut attributes);
    request.process_istio_attr(&mut attributes);


    let server_name = main_config.mixer_server.to_str();
    let server_port = main_config.mixer_port as u16;


    if !check(server_name,server_port, attributes) {
        ngx_log!("rust check denied");
        return NGX_DECLINED as ngx_int_t;
    }

    ngx_log!("rust check allowed");
    return NGX_OK as ngx_int_t;
}
