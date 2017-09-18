MODULE_NAME=ngx_http_istio_mixer_module
MODULE_PROJ_NAME=ngx-http-istio-mixer
NGX_DEBUG="--with-debug"


linux-setup:
	make -f nginx.mk MODULE_PROJ_NAME=$(MODULE_PROJ_NAME) linux-setup


linux-module:
	make -f nginx.mk MODULE_PROJ_NAME=$(MODULE_PROJ_NAME) linux-module

linux-shell:
	make -f nginx.mk MODULE_PROJ_NAME=$(MODULE_PROJ_NAME) linux-shell


nginx-setup:
	make -f nginx.mk MODULE_PROJ_NAME=$(MODULE_PROJ_NAME) nginx-setup

nginx-module:
	make -f nginx.mk MODULE_PROJ_NAME=$(MODULE_PROJ_NAME) nginx-module


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
