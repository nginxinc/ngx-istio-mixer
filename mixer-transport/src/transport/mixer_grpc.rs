extern crate grpc;
extern crate futures;


use grpc::RequestOptions;
use transport::server_info::MixerInfo;
use transport::status:: { Status, StatusCodeEnum };
use mixer_grpc::service_grpc::MixerClient;
use mixer_grpc::service_grpc::Mixer;
use mixer_grpc::check:: { CheckRequest, CheckResponse };
use attribute::attr_wrapper::AttributeWrapper;
use futures::future:: { Future,ok,err};



// transport
pub trait Transport {

    fn get_attributes(&self) -> &AttributeWrapper;

    fn check(&self,request: CheckRequest) ->  Box<Future<Item = CheckResponse, Error=Status>> ;
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

    fn check(&self,request: CheckRequest) ->  Box<Future<Item = CheckResponse, Error=Status>> {

        let client = MixerClient::new_plain( self.mixer_info.get_server_name(), self.mixer_info.get_server_port() , Default::default()).expect("init");

        //log(&format!("sending check request: {:?}",request));


        Box::new(client.check(RequestOptions::new(), request).join_metadata_result().then( |result| {

            //       log(&format!("mixer check {:?}",result));
            match result   {
                Ok(response) =>  {
                    let (_m1, check_response, _m2) = response;
                    //log(&format!("received check response {:?}",check_response));
                    return ok::<CheckResponse,Status>(check_response);
                    // need function pointer
                },

                Err(_error)  =>  {
                    // TODO: fix log error to nginx error logger
                    //log(&format!("error calling check {:?}",error));
                    return err::<CheckResponse,Status>(Status::with_code(StatusCodeEnum::CANCELLED))
                }

            }
        }))


    }
}

