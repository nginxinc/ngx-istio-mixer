extern crate protoc_rust_grpc;
use std::env;

const MIXER_GRPC_OUT: &str  = "src/mixer";


fn generate_protoc() {
     protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: MIXER_GRPC_OUT,
        includes: &["../protobuf"],
        input: &["../protobuf/mixer/v1/check.proto","../protobuf/mixer/v1/report.proto",
            "../protobuf/mixer/v1/attributes.proto",
            "../protobuf/google/rpc/status.proto","../protobuf/mixer/v1/service.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");
}

fn main() {

    // We assume that we are in a valid directory.
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    generate_protoc();
}
