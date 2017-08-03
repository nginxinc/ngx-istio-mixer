GCLOUD_PROJECT = istio-stuff
TAG = 0.1
NAME = gcr.io/$(GCLOUD_PROJECT)/ngx-mixer-dev
NGINX_VER = 1.11.13

run:
	docker run -it -v /Users/sehyochang/git/istio:/src gcr.io/istio-stuff/ngx-mixer-dev:0.1  /bin/bash

module:
    cd /src/nginx-${NGINX_VER}
	./configure --add-dynamic-module=../ngx-http-istio-mixer --with-compat --with-file-aio --with-threads --with-http_addition_module \
	--with-http_auth_request_module --with-http_dav_module --with-http_flv_module --with-http_gunzip_module --with-http_gzip_static_module \
	--with-http_mp4_module --with-http_random_index_module --with-http_realip_module --with-http_secure_link_module --with-http_slice_module \
	--with-http_ssl_module --with-http_stub_status_module --with-http_sub_module --with-http_v2_module --with-mail \
	--with-mail_ssl_module --with-stream --with-stream_realip_module --with-stream_ssl_module --with-stream_ssl_preread_module \
	--with-cc-opt='-g -O2 -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC' --with-ld-opt='-Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie'
	make modules

build:
	docker build -t $(NAME):$(TAG) .

push:
	gcloud docker --server=gcr.io -- push $(NAME):$(TAG) 

test:
	docker run --rm -p 9000:9000  $(NAME):$(TAG)

