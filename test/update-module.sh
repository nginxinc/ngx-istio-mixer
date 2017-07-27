#!/bin/bash
make modules
sudo cp objs/ngx_http_istio_mixer_module.so /usr/local/nginx/modules/
sudo /usr/local/nginx/sbin/nginx -s stop
sudo /usr/local/nginx/sbin/nginx
curl localhost/hello
