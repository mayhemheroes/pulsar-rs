# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Dependencies
RUN apt update && apt install -y cmake build-essential

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/message_codec /message_codec && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv fuzz/target/release/message_codec /message_codec_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /message_codec /message_codec_no_inst /
