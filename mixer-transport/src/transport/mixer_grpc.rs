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
        println!("sending check request, {:?}",request);


        let result = client.check(RequestOptions::new(), request).wait();

       // println!("received result check {:?}",result);
        println!("received result check {:?}", result);
        match result {
            Ok(response) => {
                let (_m1, check_response, _m2) = response;
                return Box::new(ok::<CheckResponse, Status>(check_response));
            },

            Err(_error) => {
                // TODO: fix log error to nginx error logger
                println!("error calling check {:?}", _error);
                return Box::new(err::<CheckResponse, Status>(Status::with_code(StatusCodeEnum::CANCELLED)))
            }
        }


    }
}

