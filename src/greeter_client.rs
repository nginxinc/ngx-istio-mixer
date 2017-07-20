extern crate grpc;
extern crate futures;
extern crate libc;


use std::env;
use helloworld_grpc::GreeterClient;
use helloworld::HelloRequest;
use helloworld_grpc::Greeter;

#[no_mangle]
pub extern fn hello_rust() -> *const u8 {


    let client = GreeterClient::new_plain("localhost", 50051, Default::default()).unwrap();

    let mut req = HelloRequest::new();
    req.set_name(String::from("Hello, world!"));

    let resp = client.say_hello(grpc::RequestOptions::new(), req);

    resp.wait();

    "Hello, world!\0".as_ptr()
}
