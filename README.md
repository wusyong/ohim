# Ohim - A WebAssembly-based script engine

> [!WARNING]
> Current porject is still Work In Progress.

Ohim is currently a proof of concept, but aim to evolve into full script engine that follows [DOM Standard](https://dom.spec.whatwg.org/).
See this [blog post](https://wusyong.github.io/posts/wasmtime-script-engine/) to learn what we intend and how to
achieve. We will collect feedback and start with minimum viable `Node` tree soon.

## Demo

### Run with Rust Guest
```
cargo build --target wasm32-wasip2 -p test
cargo run
```

### Run with Golang Guest
```
cd go-guest
./build.sh
cargo run
```
