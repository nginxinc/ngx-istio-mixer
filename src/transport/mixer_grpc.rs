extern crate grpc;
extern crate futures;


use grpc::RequestOptions;
use transport::server_info::MixerInfo;
use mixer::service_grpc::MixerClient;
use mixer::service_grpc::Mixer;
use mixer::check:: { CheckRequest, CheckResponse };
use attribute::attr_wrapper::AttributeWrapper;

use ngx_rust::nginx_http::log;
use futures::Future;



// transport
pub trait Transport {

    fn get_attributes(&self) -> &AttributeWrapper;

    fn check(&self,request: CheckRequest) ->  grpc::SingleResponse<CheckResponse> ;
}



pub struct GrpcTransport {

    mixer_info: MixerInfo,
    attributes: AttributeWrapper
}

impl GrpcTransport {

    pub fn new(info: MixerInfo, attributes: AttributeWrapper) -> GrpcTransport {
        GrpcTransport {
            mixer_info: info,
            attributes
        }
    }
}

impl  Transport for GrpcTransport {


    fn get_attributes(&self) -> &AttributeWrapper {
        &self.attributes
    }

    fn check(&self,request: CheckRequest) ->  grpc::SingleResponse<CheckResponse> {

        let client = MixerClient::new_plain( self.mixer_info.get_server_name(), self.mixer_info.get_server_port() , Default::default()).expect("init");

        log(&format!("sending check request: {:?}",request));

        /*
        client.check(RequestOptions::new(), request).then( |result| {

            //       log(&format!("mixer check {:?}",result));
            match result   {
                Ok(response) =>  {
                    let (m1, check_response, m2) = response;
                    log(&format!("received check response {:?}",check_response));
                    return check_response;
                    // need function pointer
                },

                Err(err)  =>  {
                    // TODO: fix log error to nginx error logger
                    log(&format!("error calling check {:?}",err));
                    return CheckResponse::new();
                }

            }
        }) */

        client.check(RequestOptions::new(), request)


    }
}

