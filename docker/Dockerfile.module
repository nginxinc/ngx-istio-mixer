FROM nginmesh/ngx_http_istio_mixer_module-base:latest

MAINTAINER Sehyo Chang "sehyo@nginx.com"

# this is coming from context which is from /build/context not root of the source
ADD ./mixer-ngx /src/mixer-ngx
ADD ./mixer-transport /src/mixer-transport
ADD ./mixer-tests /src/mixer-tests
ADD ./module /src/module
RUN cd /src;make nginx-module