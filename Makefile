GCLOUD_PROJECT = istio-stuff
RUST_COMPILER_TAG = 0.1
RUST_TOOL = gcr.io/$(GCLOUD_PROJECT)/ngx-mixer-dev:${RUST_COMPILER_TAG}
NGINX_VER = 1.11.13
MODULE_NAME=ngx_http_istio_mixer_module
MODULE_LIB=objs/${MODULE_NAME}.so
NGX_LOCAL=/usr/local/nginx
NGX_DEBUG="--with-debug"
DARWIN_NGINX=nginx-darwin
LINUX_NGINX=nginx-linux
export ROOT_DIR=$(shell dirname $$PWD)
MODULE_PROJ_NAME=ngx-http-istio-mixer
TEST_URL=localhost/test2

linux-source:
	wget http://nginx.org/download/nginx-${NGINX_VER}.tar.gz
	tar zxf nginx-${NGINX_VER}.tar.gz
	mv nginx-${NGINX_VER} ${LINUX_NGINX}
	mv ${LINUX_NGINX} nginx
	rm nginx-${NGINX_VER}.tar.gz*



# run nginx configure steps, this must be run after lx-compiler
docker-lx-configure:
	cd nginx/${LINUX_NGINX}; \
	./configure --add-dynamic-module=../../module  \
	    --with-compat --with-file-aio --with-threads --with-http_addition_module \
	    --with-http_auth_request_module --with-http_dav_module --with-http_flv_module \
	    --with-http_gunzip_module --with-http_gzip_static_module --with-http_mp4_module \
	    --with-http_random_index_module --with-http_realip_module --with-http_secure_link_module \
	    --with-http_slice_module --with-http_ssl_module --with-http_stub_status_module \
	    --with-http_sub_module --with-mail --with-mail_ssl_module \
	    --with-stream --with-stream_realip_module --with-stream_ssl_module --with-stream_ssl_preread_module \
	    --with-cc-opt='-g -O2 -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC' \
	    --with-ld-opt='-Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie'

linux-configure:
	docker run -it -v ${ROOT_DIR}:/src -w /src/${MODULE_PROJ_NAME} ${RUST_TOOL} make docker-lx-configure


# generate module. this will produce .so which will used by agent build
docker-lx-gen-module:
	cd nginx/${LINUX_NGINX}; \
	make modules


linux-module:
	docker run -it -v ${ROOT_DIR}:/src -w /src/${MODULE_PROJ_NAME} ${RUST_TOOL} make docker-lx-gen-module

darwin-source:
	wget http://nginx.org/download/nginx-${NGINX_VER}.tar.gz
	tar zxf nginx-${NGINX_VER}.tar.gz
	mv nginx-${NGINX_VER} ${DARWIN_NGINX}
	mv ${DARWIN_NGINX} nginx
	rm nginx-${NGINX_VER}.tar.gz*

darwin-configure:
	cd nginx/${DARWIN_NGINX}; \
    ./configure --add-dynamic-module=../../module

darwin-setup:   darwin-source darwin-configure


# build module locally in mac
darwin-module:
	cd nginx/${DARWIN_NGINX}; \
	make modules;


# restart local nginx in the mac
darwin-test-setup:
	sudo cp nginx/${DARWIN_NGINX}/${MODULE_LIB} ${NGX_LOCAL}/modules
	sudo ${NGX_LOCAL}/sbin/nginx -s stop
	sudo ${NGX_LOCAL}/sbin/nginx



# run simple test against local ninx
darwin-test:
	curl --header "X-ISTIO-SRC-IP: 10.43.252.73" --header "X-ISTIO-SRC-UID: kubernetes://productpage-v1-3990756607-0d23m.default" ${TEST_URL}

# build and run test in mac
darwin-test-all: darwin-module darwin-test-setup


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
	rm -f src/report.rs
	rm -f src/service_grpc.rs


super_clean: clean
	rm -rf nginx/*


report:
	cargo build --bin report_client

test:	
	cargo test -- --nocapture
