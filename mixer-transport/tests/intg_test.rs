extern crate ngx_mixer_transport;
extern crate grpc;
extern crate futures;

use ngx_mixer_transport::istio_client::mixer_client_wrapper::MixerClientWrapper ;
use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::transport::mixer_grpc::GrpcTransport;
use ngx_mixer_transport::transport::server_info::MixerInfo;
use futures::future::Future;

#[test]
fn test_check() {


    let info = MixerInfo { server_name: String::from("localhost"), server_port: 9002};
    let attributes = AttributeWrapper::new();

    let transport = GrpcTransport::new(info,attributes);

    let client = MixerClientWrapper::new();

    let result = client.check(transport).wait();

    println!("result, {:?}",result);


    assert_eq!(true,true,"verify check");
}