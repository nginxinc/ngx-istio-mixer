
compile:
	cargo rustc --lib -- --emit obj
	mv target/debug/deps/grpc_examples-*.o ../nginx-1.11.13/objs/addon/nginx-hello-rpc-rust/greeter_client.o

lib:
	cargo build
	mv target/debug/libgreeter.dylib ../nginx-1.11.13/objs/addon/nginx-hello-rpc-rust
clean:
	rm src/hello*.rs
	rm src/route*.rs
