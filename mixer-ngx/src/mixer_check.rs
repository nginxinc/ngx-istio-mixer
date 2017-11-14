use futures::future::Future;


use ngx_rust::bindings::ngx_http_request_s;
use ngx_rust::bindings::ngx_int_t;
use ngx_rust::bindings::{ NGX_OK, NGX_HTTP_UNAUTHORIZED };


use ngx::server_config::ngx_http_mixer_srv_conf_t;
use ngx::main_config::ngx_http_mixer_main_conf_t;


use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::transport::status:: { StatusCodeEnum };

use ngx_mixer_transport::istio_client::mixer_client_wrapper::MixerClientWrapper ;
use ngx_mixer_transport::transport::mixer_grpc::GrpcTransport;
use ngx_mixer_transport::transport::server_info::MixerInfo;

use ngx::config::MixerConfig;


lazy_static! {
    static ref DEFAULT_MIXER_CLIENT: MixerClientWrapper = MixerClientWrapper::new();
}




// perform check
pub fn check(request: &ngx_http_request_s,server_name: &str,server_port: u16, attr: AttributeWrapper) -> bool {


    let info = MixerInfo { server_name: String::from(server_name), server_port: server_port};

    let transport = GrpcTransport::new(info,attr);
    let result = DEFAULT_MIXER_CLIENT.check(transport).wait();

    match result {
        Ok(_) => return true,
        Err(error) => {
            ngx_http_debug!(request,"rust check transport failed: {:?}", error);

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
                                           srv_conf_option: Option<&ngx_http_mixer_srv_conf_t>)  -> ngx_int_t {

    ngx_http_debug!(request,"mixer check handler called");

    let mut attributes = AttributeWrapper::new();

    if let Some(srv_conf) = srv_conf_option {
        ngx_http_debug!(request,"calling mixer server to validate check");
        srv_conf.process_istio_attr(&mut attributes);
    }
    request.process_istio_attr(&mut attributes);


    let server_name = main_config.mixer_server.to_str();
    let server_port = main_config.mixer_port as u16;


    if !check(request,server_name,server_port, attributes) {
        ngx_http_debug!(request,"mixer check denied");
        return NGX_HTTP_UNAUTHORIZED as ngx_int_t;
    }

    ngx_http_debug!(request,"mixer check allowed");
    return NGX_OK as ngx_int_t;

}
