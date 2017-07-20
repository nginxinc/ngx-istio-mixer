extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["protobuf"],
        input: &["protobuf/mixer/v1/check.proto","protobuf/mixer/v1/quota.proto","protobuf/mixer/v1/report.proto","protobuf/mixer/v1/attributes.proto","protobuf/google/rpc/status.proto","protobuf/mixer/v1/service.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");
}
