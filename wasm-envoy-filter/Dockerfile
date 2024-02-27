FROM rust:1.76 as builder
WORKDIR /usr/src/wasm-envoy-filter
COPY . .
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release

FROM scratch
COPY --from=builder /usr/src/wasm-envoy-filter/target/wasm32-wasi/release/wasm_envoy_filter.wasm /etc/envoy/plugin.wasm