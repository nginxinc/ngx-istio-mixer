NGINX_VER = 1.13.6
UNAME_S := $(shell uname -s)
GIT_COMMIT=$(shell git rev-parse --short HEAD)
NGX_DEBUG="--with-debug"
export MODULE_DIR=${PWD}
DOCKER_USER=101
MIXER_TRANSPORT_CRATE=mixer-transport
RUST_COMPILER_TAG = 1.20.0-B
NGINX_TAG=1.13.5
NGX_MODULES = --with-compat  --with-threads --with-http_addition_module \
     --with-http_auth_request_module   --with-http_gunzip_module --with-http_gzip_static_module  \
     --with-http_random_index_module --with-http_realip_module --with-http_secure_link_module \
     --with-http_slice_module  --with-http_stub_status_module --with-http_sub_module \
     --with-stream --with-stream_realip_module --with-stream_ssl_preread_module
ifeq ($(UNAME_S),Linux)
    NGINX_SRC += nginx-linux
    NGX_OPT= $(NGX_MODULES) \
       --with-file-aio
       --with-cc-opt='-g -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC' \
       --with-ld-opt='-Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie'
endif
ifeq ($(UNAME_S),Darwin)
    NGINX_SRC += nginx-darwin
    NGX_OPT= $(NGX_MODULES)
endif
DOCKER_MODULE_IMAGE = nginmesh/${MODULE_NAME}
DOCKER_MODULE_BASE_IMAGE = nginmesh/${MODULE_NAME}-base
DOCKER_MODULE_NGINX_BUILD_IMAGE = nginmesh/${MODULE_NAME}-ngx-build
DOCKER_MODULE_NGINX_BASE_IMAGE= nginmesh/${MODULE_NAME}-ngx-base
DOCKER_RUST_IMAGE = nginmesh/ngx-rust-tool:${RUST_COMPILER_TAG}
DOCKER_NGIX_IMAGE = nginmesh/nginx-dev:${NGINX_TAG}
DOCKER_MIXER_IMAGE = nginmesh/ngix-mixer:1.0
MODULE_SO_DIR=nginx/nginx-linux/objs
MODULE_SO_BIN=${MODULE_SO_DIR}/${MODULE_NAME}.so
NGINX_BIN=${MODULE_SO_DIR}/nginx
MODULE_SO_HOST=config/modules/${MODULE_NAME}.so
NGINX_SO_HOST=config
DOCKER_BUILD_TOOL=docker run -it --rm -v ${ROOT_DIR}:/src -w /src/${MODULE_PROJ_NAME} ${DOCKER_RUST_IMAGE}
DOCKER_NGINX_TOOL=docker run -it --rm -v ${ROOT_DIR}:/src -w /src/${MODULE_PROJ_NAME} ${DOCKER_NGIX_IMAGE}
DOCKER_NGINX_NAME=nginx-test
DOCKER_NGINX_EXEC=docker exec -it ${DOCKER_NGINX_NAME}
DOCKER_NGINX_EXECD=docker exec -d ${DOCKER_NGINX_NAME}
DOCKER_NGINX_DAEMON=docker run -d -p 8000:8000  --privileged --name  ${DOCKER_NGINX_NAME} \
    --sysctl net.ipv4.ip_nonlocal_bind=1 \
    --sysctl net.ipv4.ip_forward=1 \
	-v ${MODULE_DIR}/config/modules:/etc/nginx/modules \
	-v ${MODULE_DIR}:/src  -w /src   ${DOCKER_MODULE_NGINX_BASE_IMAGE}:latest


# this need to be invoked before any build steps
# set up dependencies
setup:
	rm -rf build/crates
	mkdir build/crates
	tar zxf build/vendor/protoc.zip -C build/crates
	tar zxf build/vendor/ngx-rust-tar.zip -C build/crates


nginx-build:
	cd nginx/${NGINX_SRC}; \
	./configure --prefix=${PWD}/nginx/install $(NGX_OPT); \
	make;


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


# setup nginx container for testing
# copies the configuration and modules
# start test services
test-nginx-setup:
	cp config/nginx.conf /etc/nginx
	rm -rf /etc/nginx/conf.d/*
	cp config/conf.d/* /etc/nginx/conf.d
	node tests/services/http.js 9100 > u1.log 2> u1.err &
#	tests/tproxy.sh &
	nginx -s reload


# run integrated test
test-intg:
	cargo +stable test --color=always intg -- --nocapture

# remove nginx container
test-nginx-clean:
	docker rm -f  ${DOCKER_NGINX_NAME} || true


test-nginx-only: test-nginx-clean
	$(DOCKER_NGINX_DAEMON)
	$(DOCKER_NGINX_EXECD) make test-nginx-setup > make.out
	sleep 1


test-nginx-log:
	docker logs -f nginx-test


test-nginx-full:	build-module test-nginx-only

# invoke http service
test-http:
	curl localhost:8000


copy-module:
	docker rm -v ngx-copy || true
	docker create --name ngx-copy ${DOCKER_MODULE_IMAGE}:latest
	docker cp ngx-copy:/src/${MODULE_SO_BIN} ${MODULE_SO_HOST}
	docker rm -v ngx-copy

# build module using docker
# we copy only necessary context to docker daemon (src and module directory)
build-module-docker:
	rm -rf build/context
	mkdir build/context
	cp build/Dockerfile.module build/context
	cp -r mixer-ngx build/context
	cp -r mixer-transport build/context
	cp -r module build/context
	docker build -f ./build/context/Dockerfile.module -t ${DOCKER_MODULE_IMAGE}:latest ./build/context

# build module and deposit in the module directory
build-module: build-module-docker copy-module

# build base container image that pre-compiles rust and nginx modules
build-base:	super_clean
	docker build -f ./build/Dockerfile.base -t ${DOCKER_MODULE_BASE_IMAGE}:${GIT_COMMIT} .
	docker tag ${DOCKER_MODULE_BASE_IMAGE}:${GIT_COMMIT} ${DOCKER_MODULE_BASE_IMAGE}:latest


copy-ngx-exec:
	docker rm -v ngx-copy || true
	docker create --name ngx-copy ${DOCKER_MODULE_NGINX_BUILD_IMAGE}:latest
	docker cp ngx-copy:/src/${NGINX_BIN} ${NGINX_SO_HOST}
	docker rm -v ngx-copy


build-nginx-base:
	docker build -f ./build/Dockerfile.build-nginx -t ${DOCKER_MODULE_NGINX_BASE_IMAGE}:${GIT_COMMIT} .
	docker tag ${DOCKER_MODULE_NGINX_BASE_IMAGE}:${GIT_COMMIT} ${DOCKER_MODULE_NGINX_BASE_IMAGE}:latest



run-base-image:
	docker run -it --rm  ${DOCKER_MODULE_BASE_IMAGE}:latest /bin/bash


run-ngx-image:
	docker run -it --rm  ${DOCKER_MODULE_NGINX_BASE_IMAGE}:latest /bin/ash



# copy dependent modules that must be load locally. they are assume to be checked as peer directory
# later, they should be clone directly from github repo
zip-dependent-modules:
	cd ..;tar --exclude ".git" --exclude ".idea" -zcvf  ngx-rust-tar.zip ngx-rust
	cd ..;tar --exclude ".git" --exclude ".idea" -zcvf  protoc.zip  grpc-rust
	cp ../ngx-rust-tar.zip .
	cp ../protoc.zip .


watch-mixer:
	 kubectl logs -f $(kubectl get pod -l istio=mixer -n istio-system -o jsonpath='{.items[0].metadata.name}')  -n istio-system -c mixer	
