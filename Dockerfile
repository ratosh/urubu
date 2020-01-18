FROM ubuntu:18.04

RUN apt-get update \
    && apt-get install -y \
        cargo

WORKDIR /app

COPY ./ /app

RUN cargo build --release

RUN apt-get update \
    && apt-get install -y \
        linux-tools

WORKDIR /app
RUN echo 0 > /proc/sys/kernel/kptr_restrict
RUN perf record --call-graph dwarf -- cargo run --release
RUN perf report