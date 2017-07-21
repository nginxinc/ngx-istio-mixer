/**
  test sending of report
 */


extern crate mixer;
extern crate grpc;
extern crate futures;
extern crate libc;


use std::env;
use std::collections::HashMap;
use mixer::service_grpc::MixerClient;
use mixer::report::ReportRequest;
use mixer::attributes::Attributes;
use mixer::service_grpc::Mixer;

static REQUEST_HEADER: i32 = 0;
static TARGET_SERVICE: i32 = 1;

fn main() {

    println!("creating mixer client at local host");

    let client = MixerClient::new_plain("localhost", 9091, Default::default()).expect("init");


    let mut requests = Vec::new();
    let mut req = ReportRequest::new();
    let mut attr = Attributes::new();
    //attr.set_string_attributes("")
    req.set_request_index(0);

    let mut dictValues: HashMap<i32,String> = HashMap::new();
    dictValues.insert(REQUEST_HEADER,String::from("request.headers"));
    dictValues.insert(TARGET_SERVICE,String::from("target.service"));
    attr.set_dictionary(dictValues);

    let mut stringValues: HashMap<i32,String> = HashMap::new();
    stringValues.insert(TARGET_SERVICE,String::from("reviews.default.svc.cluster.local"));
    stringValues.insert(REQUEST_HEADER,String::from("content-length:0"));
    attr.set_string_attributes(stringValues);

    req.set_attribute_update(attr);


    requests.push(req);


    let resp = client.report(grpc::RequestOptions::new(), grpc::StreamingRequest::iter(requests));
    ;


    println!("send report request: {} count",resp.wait_drop_metadata().count());
}
