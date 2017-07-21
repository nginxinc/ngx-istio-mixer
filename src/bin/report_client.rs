/**
  test sending of report
 */


extern crate mixer;
extern crate grpc;
extern crate futures;
extern crate libc;


use std::env;
use mixer::service_grpc::MixerClient;
use mixer::report::ReportRequest;


fn main() {
    let client = MixerClient::new_plain("localhost", 50051, Default::default()).unwrap();

    let mut req = ReportRequest::new();
    // req.set_name(String::from("Hello, world!"));

    //let resp = client.check(grpc::RequestOptions::new(), req);

    //resp.wait();


    println!("send report request");
}
