extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;
extern crate tls_api;
extern crate time;
extern crate base64;
extern crate ngx_rust;

#[macro_use]
extern crate lazy_static;

pub mod check;
pub mod attributes;
pub mod status;
pub mod report;

pub mod service_grpc;

pub mod mixer_client;
pub mod attr_wrapper;
pub mod global_dict;
pub mod message_dict;
pub mod encode;

#[cfg(test)]
#[path = "./global_dict_test.rs"]
mod global_test_dict;

#[cfg(test)]
#[path = "./message_dict_test.rs"]
mod message_dict_test;

#[cfg(test)]
#[path = "./attr_wrapper_test.rs"]
mod attr_wrapper_test;