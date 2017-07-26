
restart:
	sudo /usr/local/nginx/sbin/nginx -s stop

start:
	sudo /usr/local/nginx/sbin/nginx

clean:
	cargo clean
	rm -f src/attributes.rs
	rm -f src/status.rs
	rm -f src/check.rs
	rm -f src/quota.rs
	rm -f src/service_grpc.rs
	rm -f src/bindings.rs


report:
	cargo build --bin report_client
