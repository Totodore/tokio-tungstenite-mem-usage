# Benchmark memory usage of tokio-tungstenite between 0.24 and 0.25

The source code is the official ping/pong example from the tokio-tungstenite repository.

The results are obtained with `heaptrack`, a heap memory profiler.

## tokio-tungstenite 0.24.0


## tokio-tungstenite 0.25.0

## Reproduction steps
* install [k6](https://k6.io/open-source/)
* `cargo run -r -p tokio-tungstenite-0-24`
* `k6 run k6-script.js`
* `cargo run -r -p tokio-tungstenite-0-25`
* `k6 run k6-script.js`
