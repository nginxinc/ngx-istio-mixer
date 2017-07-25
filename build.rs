extern crate protoc_rust_grpc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {


    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["protobuf"],
        input: &["protobuf/mixer/v1/check.proto","protobuf/mixer/v1/quota.proto","protobuf/mixer/v1/report.proto","protobuf/mixer/v1/attributes.proto","protobuf/google/rpc/status.proto","protobuf/mixer/v1/service.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");


    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-I/Users/sehyochang/git/istio/nginx-1.11.13/src/core")
        .clang_arg("-I/Users/sehyochang/git/istio/nginx-1.11.13/src/event")
        .clang_arg("-I/Users/sehyochang/git/istio/nginx-1.11.13/src/event/modules")
        .clang_arg("-I/Users/sehyochang/git/istio/nginx-1.11.13/src/os/unix")
        .clang_arg("-I/Users/sehyochang/git/istio/nginx-1.11.13/objs")
        .clang_arg("-I/Users/sehyochang/git/istio/nginx-1.11.13/src/http")
        .clang_arg("-I/Users/sehyochang/git/istio/nginx-1.11.13/src/http/modules")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    //let out_path = PathBuf::from(env::var("/Users/sehyochang/git/istio/ngx-http-istio-mixer/src/").unwrap());
    bindings
        .write_to_file("/Users/sehyochang/git/istio/ngx-http-istio-mixer/src/bindings.rs")
        .expect("Couldn't write bindings!");
}
