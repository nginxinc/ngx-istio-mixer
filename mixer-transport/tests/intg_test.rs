extern crate ngx_mixer_transport;
extern crate grpc;
extern crate futures;

use ngx_mixer_transport::istio_client::mixer_client_wrapper::MixerClientWrapper ;
use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::transport::mixer_grpc::GrpcTransport;
use ngx_mixer_transport::transport::server_info::MixerInfo;
use ngx_mixer_transport::transport::status::StatusCodeEnum;
use futures::future::Future;

//

#[test]
fn intg_check_empty_request() {


    let info = MixerInfo { server_name: String::from("localhost"), server_port: 9091};
    let attributes = AttributeWrapper::new();

    let transport = GrpcTransport::new(info,attributes);

    let client = MixerClientWrapper::new();

    let result = client.check(transport).wait();

    println!("result, {:?}",result);

    match result  {
        Ok(response) =>  assert!(true,"succeed"),
        Err(error)  => assert!(false,"failed check")
    }
}

