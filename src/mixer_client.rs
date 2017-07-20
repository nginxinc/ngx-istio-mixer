extern crate grpc;
extern crate futures;
extern crate libc;


use std::env;
use service_grpc::MixerClient;
use check::CheckRequest;


#[no_mangle]
pub extern fn hello_rust() -> *const u8 {


    let client = MixerClient::new_plain("localhost", 50051, Default::default()).unwrap();

    let mut req = CheckRequest::new();
   // req.set_name(String::from("Hello, world!"));

    //let resp = client.check(grpc::RequestOptions::new(), req);

   // resp.wait();

    "Hello, world!\0".as_ptr()
}
