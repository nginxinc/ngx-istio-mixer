/**
  test sending of report
 */


extern crate mixer;
extern crate grpc;
extern crate futures;
extern crate libc;

use std::collections::HashMap;
use mixer::service_grpc::MixerClient;
use mixer::report::ReportRequest;
use mixer::attributes::Attributes;
use mixer::service_grpc::Mixer;

static REQUEST_HEADER: i32 = 0;
static TARGET_SERVICE: i32 = 1;
static REQUEST_HOST: i32 = 2;
static USER_AGENT: i32 = 3;

fn main() {

    println!("creating mixer client at local host");

    let client = MixerClient::new_plain("localhost", 9091, Default::default()).expect("init");


    let mut requests = Vec::new();
    let mut req = ReportRequest::new();
    let mut attr = Attributes::new();
    //attr.set_string_attributes("")
    req.set_request_index(0);

    let mut dict_values: HashMap<i32,String> = HashMap::new();
    dict_values.insert(REQUEST_HEADER,String::from("request.headers"));
    dict_values.insert(TARGET_SERVICE,String::from("target.service"));
    dict_values.insert(REQUEST_HOST,String::from("request.host"));
    attr.set_dictionary(dict_values);

    let mut string_values: HashMap<i32,String> = HashMap::new();
    string_values.insert(TARGET_SERVICE,String::from("reviews.default.svc.cluster.local"));
    string_values.insert(REQUEST_HOST,String::from("test.com"));
    attr.set_string_attributes(string_values);


    req.set_attribute_update(attr);


    requests.push(req);


    let resp = client.report(grpc::RequestOptions::new(), grpc::StreamingRequest::iter(requests));
    ;


    println!("send report request: {} count",resp.wait_drop_metadata().count());
}
