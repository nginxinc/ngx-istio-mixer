# Nginx Mixer Module

To integrate with Istio Mixer

## Usage

<TBD>

## Check out Nginx Rust Module

```bash
giut clone git@github.com:nginxinc/ngx-rust.git
```

Rust module needs to be check out at same level as this project.
Follow instruction in Rust module and configure for each of the target OS.


## Install CLang for bindgen

Install Clang at 

https://rust-lang-nursery.github.io/rust-bindgen/requirements.html


## Checkout Rust GRPC project

Provides gRpc compile

Checkout rust-grpc, which must be check out at the same level as this project

```bash
giut clone git@github.com:stepancheg/grpc-rust.git
```


## Configure and Build

Before building, it must be configured for each of the target OS.

### For Linux

To configure:

```bash
make linux-setup
```

Generating module:


```bash
make linux-module
```

Generated module is founded at:

```bash
ls nginx/nginx-linux/objs/ngx_http_istio_mixer_module.so
```


### For Mac

To configure:

```bash
make darwin-setup
```

Generating module:


```bash
make darwin-module
```


### Run mixer unit test.

Configuration step must be done before.

`cargo run --bin report_client`

