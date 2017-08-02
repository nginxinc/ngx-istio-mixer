GCLOUD_PROJECT = istio-stuff
TAG = 0.1
NAME = gcr.io/$(GCLOUD_PROJECT)/ngx-mixer-dev

compile:
	docker run -it -v /Users/sehyochang/git/istio:/src gcr.io/istio-stuff/ngx-mixer-dev:0.1  /bin/bash

build:
	docker build -t $(NAME):$(TAG) .

push:
	gcloud docker --server=gcr.io -- push $(NAME):$(TAG) 

test:
	docker run --rm -p 9000:9000  $(NAME):$(TAG)

