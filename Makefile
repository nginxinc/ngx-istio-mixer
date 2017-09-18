MODULE_NAME=ngx_http_istio_mixer_module
MODULE_PROJ_NAME=ngx-http-istio-mixer
NGX_DEBUG="--with-debug"
include nginx.mk


clean:
	cargo clean
	rm -f src/attributes.rs
	rm -f src/status.rs
	rm -f src/check.rs
	rm -f src/quota.rs
	rm -f src/report.rs
	rm -f src/service_grpc.rs


super_clean: clean
	rm -rf nginx/*


report:
	cargo build --bin report_client

test:
	cargo test -- --nocapture
