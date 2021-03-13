FROM ekidd/rust-musl-builder:1.50.0 AS builder
WORKDIR /home/rust

# The 'chown' is necessary due to https://github.com/emk/rust-musl-builder/issues/26
RUN sudo chown -R rust:rust .

# Rust doesn't yet have a way to cache build dependencies as Docker layers
# (see https://github.com/rust-lang/cargo/issues/2644), so we use this hack 
# from https://blog.mgattozzi.dev/caching-rust-docker-builds/ so that we get
# at least a little bit of caching. We'd like to use 'cargo-chef', but the rust-musl-builder
# doens't play nicely with 'cargo-chef' (see https://github.com/LukeMathWalker/cargo-chef/issues/43)
# WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING
# Unfortunately, Docker doesn't seem to handle this well - any time the Cargo.toml file is updated,
# you must make at least one change to the code files else Docker will produce an empty image that
# does nothing!!!!
# WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING WARNING

RUN cargo install --version 0.2.0 cargo-build-deps
RUN mkdir -p testsuite/src
COPY Cargo.lock Cargo.toml ./
COPY testsuite/Cargo.toml testsuite/
RUN echo "fn main() {}" > testsuite/src/main.rs
# Make the mtime of the dummy main.rs file Jan 1, 1970, so that our new code that we copy in will trigger a cargo build
# See: https://github.com/rust-lang/cargo/issues/7181
RUN touch -a -m -t 197001010000.00 testsuite/src/main.rs
RUN cargo build-deps --release
COPY testsuite testsuite
RUN cargo build --release

#  # From https://github.com/rust-lang/cargo/issues/2644#issuecomment-413838210
#  RUN mkdir -p testsuite/src
#  RUN echo "fn main() {}" > testsuite/src/main.rs
#  COPY Cargo.lock .
#  COPY Cargo.toml .
#  COPY testsuite/Cargo.toml testsuite/
#  # TODO DEBUGGING
#  # RUN cargo build --release
#  RUN cargo build
#  
#  COPY testsuite testsuite

# # TODO DEBUGGING
# # This is necessary because Cargo is trying to be fancy, but it messes up our build:
# # https://github.com/rust-lang/cargo/issues/7181
# RUN touch testsuite/src/main.rs

# TODO DEBUGGING
# RUN cargo test --release --bin testsuite
# RUN cargo build --release --bin testsuite
# RUN cargo build --bin testsuite

# ============= Execution Stage ================
FROM alpine:3.12 AS execution

WORKDIR /testsuite

# The BUILD_TIMESTAMP arg, set by build-and-run-core.sh, will have a different value every time
# Echo'ing it means that Docker will not cache any steps after this one
# This cache-busting is intentional, because (as of 2021-03-12) Docker is incorrectly caching the
#     COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/testsuite ./
# step, which results in a Docker image that does nothing.
# This bug was reported to the Docker team in: https://github.com/docker/for-linux/issues/273
ARG BUILD_TIMESTAMP
RUN echo ${BUILD_TIMESTAMP}

# Copy the code into the container
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/testsuite ./

# TODO Switch to exec command form, wrapping arguments with double-quote
CMD RUST_BACKTRACE=full ./testsuite \
    --custom-params-json="${CUSTOM_PARAMS_JSON}" \
    --kurtosis-api-socket="${KURTOSIS_API_SOCKET}" \
    --log-level="${LOG_LEVEL}"
