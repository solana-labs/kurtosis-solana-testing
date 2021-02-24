FROM ekidd/rust-musl-builder:1.50.0 AS builder
WORKDIR /home/rust

# # Required for 'hidapi' transitive dependency of the solana client
# # See: https://github.com/tendermint/tmkms/issues/257
# RUN sudo apt-get update -y
# RUN sudo apt-get install -y libudev-dev
# RUN sudo apt-get install -y linux-headers
# RUN sudo apt-get install -y linux-headers-generic
# RUN sudo apt-get install -y libhidapi-dev

## Rust doesn't yet have a way to cache build dependencies as Docker layers, so we use this hack so 
# that building doesn't take 5 minutes every time
# The 'chown' is necessary due to https://github.com/emk/rust-musl-builder/issues/26
COPY --chown=rust:rust Cargo.toml ./
COPY --chown=rust:rust Cargo.lock ./
COPY --chown=rust:rust testsuite/Cargo.toml testsuite/
COPY --chown=rust:rust testsuite/dummy_main_for_build_cache.rs testsuite/src/main.rs
RUN cargo build --release


COPY --chown=rust:rust testsuite testsuite
RUN cargo test --release --bin testsuite
RUN cargo build --release --bin testsuite

# ============= Execution Stage ================
FROM alpine:3.12 AS execution

# Copy the code into the container
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/testsuite ./

# TODO Switch to exec command form, wrapping arguments with double-quote
CMD RUST_BACKTRACE=1 ./testsuite \
    --custom-params-json="${CUSTOM_PARAMS_JSON}" \
    --kurtosis-api-socket="${KURTOSIS_API_SOCKET}" \
    --log-level="${LOG_LEVEL}"
