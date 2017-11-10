# Nginx Mixer Module

Nginx module for integrating with Istio Mixer as part of Sidecar Proxy.

The module is written using both C and R.  It depends on Nginx Rust module which provides 
Rust wrapper for Nginx core.

## Requirements

Clang is used for generating gRpc client.

https://rust-lang-nursery.github.io/rust-bindgen/requirements.html


## Set up for local development

```bash
make setup
```

This install Nginx and gRpc compiler crates necessary for building and test crate on host computer

## Unit Tests

```bash
make test-unit
```

Run all unit tests

### Building and generating Nginx module

Module generation is done using Docker to speed up build.

```bash
make build-base
```

This build base image which contains all the dependent crates and nginx core.  
It should be rebuilt if dependent crates, nginx or protobuf definition changes

```bash
make build-module
```

Generate Nginx dyanmic module which will be saved in the 'config/modules'.


### Integration Test

```bash
make test-nginx-only
```

This launches docker container with nginx configuration that can connect to outside mixer.