#!/bin/bash
cp test/config/nginx.conf /etc/nginx
rm -rf /etc/nginx/conf.d/*
cp test/config/conf.d/* /etc/nginx/conf.d
node test/services/http.js 9100 > /var/log/u1.log 2> /var/log/u1.err &
#	tests/tproxy.sh &
nginx -s reload