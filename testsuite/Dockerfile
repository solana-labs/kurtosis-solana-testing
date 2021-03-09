FROM ekidd/rust-musl-builder:1.50.0 AS builder
WORKDIR /home/rust

# Rust doesn't yet have a way to cache build dependencies as Docker layers
# (see https://github.com/rust-lang/cargo/issues/2644), so we use this hack 
# from https://blog.mgattozzi.dev/caching-rust-docker-builds/ so that we get
# at least a little bit of caching. We'd like to use 'cargo-chef', but the rust-musl-builder
# doens't play nicely with 'cargo-chef' (see https://github.com/LukeMathWalker/cargo-chef/issues/43)
# The 'chown' is necessary due to https://github.com/emk/rust-musl-builder/issues/26
# WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING
# Unfortunately, Docker doesn't seem to handle this well - any time the Cargo.toml file is updated,
# you must make at least one change to the code files else Docker will produce an empty image that
# does nothing!!!!
# WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING
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
CMD RUST_BACKTRACE=full ./testsuite \
    --custom-params-json="${CUSTOM_PARAMS_JSON}" \
    --kurtosis-api-socket="${KURTOSIS_API_SOCKET}" \
    --log-level="${LOG_LEVEL}"
