GCLOUD_PROJECT = istio-stuff
RUST_COMPILER_TAG = 0.1
RUST_TOOL = gcr.io/$(GCLOUD_PROJECT)/ngx-mixer-dev:${RUST_COMPILER_TAG}
NGINX_VER = 1.11.13
MODULE_SRC=/Users/sehyochang/git/istio


compiler:
	docker run -it -v ${MODULE_SRC}:/src ${RUST_TOOL}  /bin/bash

# run inside compiler tool
configure:
	cd /src/nginx-${NGINX_VER}; \
	./configure --add-dynamic-module=../ngx-http-istio-mixer  \
	    --with-compat --with-file-aio --with-threads --with-http_addition_module \
	    --with-http_auth_request_module --with-http_dav_module --with-http_flv_module \
	    --with-http_gunzip_module --with-http_gzip_static_module --with-http_mp4_module \
	    --with-http_random_index_module --with-http_realip_module --with-http_secure_link_module \
	    --with-http_slice_module --with-http_ssl_module --with-http_stub_status_module \
	    --with-http_sub_module --with-mail --with-mail_ssl_module \
	    --with-stream --with-stream_realip_module --with-stream_ssl_module --with-stream_ssl_preread_module \
	    --with-cc-opt='-g -O2 -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC' \
	    --with-ld-opt='-Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie'

# run inside compiler tool
module:
	cd /src/nginx-${NGINX_VER}; \
	make modules


loc-configure:
	cd ${MODULE_SRC}/nginx-${NGINX_VER}; \
	./configure --add-dynamic-module=../ngx-http-istio-mixer


# local build
build:
	cd ${MODULE_SRC}/nginx-${NGINX_VER}; \
	make modules;  \
	sudo cp objs/ngx_http_istio_mixer_module.so /usr/local/nginx/modules; \
	sudo /usr/local/nginx/sbin/nginx -s stop; \
	sudo /usr/local/nginx/sbin/nginx; \
	curl localhost/test2

mclean:
	cd ${MODULE_SRC}/nginx-${NGINX_VER}; \
	make clean

restart:
	sudo /usr/local/nginx/sbin/nginx -s stop

start:
	sudo /usr/local/nginx/sbin/nginx

setup:
	cp /usr/local/nginx/conf/nginx.conf conf


clean:
	cargo clean
	rm -f src/attributes.rs
	rm -f src/status.rs
	rm -f src/check.rs
	rm -f src/quota.rs
	rm -f src/service_grpc.rs
	rm -f src/bindings.rs

super_clean: clean mclean

report:
	cargo build --bin report_client



