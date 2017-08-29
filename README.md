# Nginx Mixer Module

## Development Set up for Mac


### Install Rust 1.18.0

Current rust version 1.19.0 has issue with linking in Mac.  Use 1.18.0 until this issue is sorted out.

First install rust at:  https://www.rust-lang.org/en-US/install.html

Then switch to Rust 1.18.0:

`rustup install 1.18.0
rustup default 1.18.0`

### Install CLang for bindgen

Install Clang at: https://rust-lang-nursery.github.io/rust-bindgen/requirements.html


// ### Install gRpc compiler
// `brew install --with-plugins grpc
// brew install --with-plugins grpc`

Checkout rust-grpc, same level as mixer:
https://github.com/stepancheg/grpc-rust

## Creating Nginx module --------???
`git clone https://github.com/nginxinc/ngx-http-istio-mixer.git`




### Check out mixer module

Check out nginx source code.  This refers to open source, but it also applies to NginxPlus.

In this example, nginx source repository is checked out at same directory level as mixer module.

`git clone https://github.com/nginx/nginx.git`

###  Link nginx repo under mixer module.

The low level nginx rust wrappr is created automatically from rust bindgen utility,.  In order do that,
the nginx repositry must be accessible from mixer module.

`cd mixer
ln -s ../nginx nginx`

### Creating mixer as static module

goto nginx module

`cd nginx-1.11.13`


`./configure --add-module=../ngx-http-istio-mixer`

make and install

`sudo make install`

### Creating mixer as dynamic module

cd nginx
./auto/configure --add-dynamic-module=../ngx-http-istio-mixer

make linux-setup
make linux-module
sudo cp objs/ngx_http_istio_mixer_module.so /usr/local/nginx/modules/
sudo /usr/local/nginx/sbin/nginx -s stop
/usr/local/nginx/sbin/nginx



### Run mixer test.

This send dummy data to mixer in order to excercise grpc interfaces

`cargo run --bin report_client`



* test4
