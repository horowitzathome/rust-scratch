# rust-scratch

This Rust program demonstrates how to build a minimal web-server with one REST endpoint
which is able to run in a *scratch* docker image on a Mac M1. 

## How to build

- `cargo build`
- `cargo build --release`
- `docker build -t rust-scratch -f .`

## How to run 

- `cargo run`
- `cargo run --release`
- `docker run -p 8080:8080 rust-scratch`

## Size of Docker image

`docker images`

| REPOSITORY | TAG | IMAGE ID |  CREATED | SIZE |
| ---------- | --- | -------- | -------- | ---- |
| rust-scratch | latest | ff9ff71bee31 | 14 seconds ago | 6.62MB |
