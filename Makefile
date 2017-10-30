MODULE_NAME=ngx_http_istio_mixer_module
MODULE_PROJ_NAME=ngx-http-istio-mixer
NGX_DEBUG="--with-debug"
include nginx.mk


clean:
	cargo clean
	rm -f ${MIXER_CRATE}/src/mixer/attributes.rs
	rm -f ${MIXER_CRATE}/src/mixer/status.rs
	rm -f ${MIXER_CRATE}/src/mixer/check.rs
	rm -f ${MIXER_CRATE}/src/mixer/quota.rs
	rm -f ${MIXER_CRATE}/src/mixer/report.rs
	rm -f ${MIXER_CRATE}/src/mixer/service_grpc.rs
	rm -f module/*.so
	rm -rf build/crates
	rm -rf build/context


super_clean: clean
	rm -rf nginx/*


report:
	cargo build --bin report_client

test:
	cargo test -- --nocapture
