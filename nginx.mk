NGINX_VER = 1.13.5
UNAME_S := $(shell uname -s)
NGX_MODULES = --with-compat  --with-threads --with-http_addition_module \
     --with-http_auth_request_module   --with-http_gunzip_module --with-http_gzip_static_module  \
     --with-http_random_index_module --with-http_realip_module --with-http_secure_link_module \
     --with-http_slice_module  --with-http_stub_status_module --with-http_sub_module \
     --with-stream --with-stream_realip_module --with-stream_ssl_preread_module

ifeq ($(UNAME_S),Linux)
    NGINX_SRC += nginx-linux
    NGX_OPT= $(NGX_MODULES) \
       --with-file-aio --with-http_ssl_module --with-stream_ssl_module  \
       --with-cc-opt='-g -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC' \
       --with-ld-opt='-Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie'
endif
ifeq ($(UNAME_S),Darwin)
    NGINX_SRC += nginx-darwin
    NGX_OPT= $(NGX_MODULES)
endif
NGX_DEBUG="--with-debug"
export ROOT_DIR=$(shell dirname $$PWD)
export MODULE_DIR=${PWD}
DOCKER_USER=101
RUST_COMPILER_TAG = 1.20.0
RUST_TOOL = nginmesh/ngx-rust-tool:${RUST_COMPILER_TAG}
MODULE_SO_DIR=nginx/nginx-linux/objs
MODULE_SO_BIN=${MODULE_SO_DIR}/${MODULE_NAME}.so
DOCKER_TOOL=docker run -it --rm -v ${ROOT_DIR}:/src -w /src/${MODULE_PROJ_NAME} ${RUST_TOOL}
DOCKER_NGINX_NAME=nginx-test
DOCKER_NGINX_EXEC=docker exec -it ${DOCKER_NGINX_NAME}
DOCKER_NGINX_EXECD=docker exec -d ${DOCKER_NGINX_NAME}
DOCKER_NGINX_DAEMON=docker run -d -p 8000:8000 --privileged --name ${DOCKER_NGINX_NAME} -v ${ROOT_DIR}:/src -w /src/${MODULE_PROJ_NAME} ${RUST_TOOL}

nginx-build:
	cd nginx/${NGINX_SRC}; \
	./configure --prefix=${PWD}/nginx/install $(NGX_OPT); \
	make; \
	make install


setup-nginx:
	mkdir -p nginx



nginx-source:	setup-nginx
	rm -rf nginx/${NGINX_SRC}
	wget http://nginx.org/download/nginx-${NGINX_VER}.tar.gz
	tar zxf nginx-${NGINX_VER}.tar.gz
	mv nginx-${NGINX_VER} ${NGINX_SRC}
	mv ${NGINX_SRC} nginx
	rm nginx-${NGINX_VER}.tar.gz*

nginx-configure:
	cd nginx/${NGINX_SRC}; \
    ./configure --add-dynamic-module=../../module $(NGX_OPT)


nginx-setup:	nginx-source nginx-configure

nginx-test:	nginx-source nginx-build


nginx-module:
	cd nginx/${NGINX_SRC}; \
	make modules;


# copy test configuration and restart
nginx-test-restart:
	docker exec -it ${DOCKER_NGINX_NAME}




# need to run inside container
linux-shell:
	${DOCKER_TOOL} /bin/bash



linux-setup:
	${DOCKER_TOOL} make nginx-setup

linux-module:
	${DOCKER_TOOL} make nginx-module

linux-copy-restart:
	cp config/nginx.conf /etc/nginx
	rm -rf /etc/nginx/conf.d/*
	cp config/mesh.conf /etc/nginx/conf.d
	cp ${MODULE_SO_BIN} /etc/nginx/modules
	node tests/services/hello.js 9100 > u1.log 2> u1.err &
	node tests/services/tcp-invoke.js 9000 dest > u1.log 2> u1.err &
	tests/prepare_proxy.sh -p 15001 -u ${DOCKER_USER} &
	nginx -s reload


linux-test-stop:
	docker stop ${DOCKER_NGINX_NAME} | xargs docker rm


linux-test-start:   linux-module linux-test-stop
	$(DOCKER_NGINX_DAEMON)
	$(DOCKER_NGINX_EXECD) make linux-copy-restart
	sleep 2

linux-test-run:
	$(DOCKER_NGINX_EXEC) cargo test


linux-test: linux-test-start linux-test-run
	docker stop ${DOCKER_NGINX_NAME} | xargs docker rm

linux-test-log:
	docker logs -f nginx-test

# open tcp connection to nginx in the containner
linux-test-nc:
	nc localhost 8000
