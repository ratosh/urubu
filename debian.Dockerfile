FROM debian:buster-slim as build

RUN apt-get update \
    && apt-get install -y \
        cargo

WORKDIR /app

COPY ./ /app

RUN cargo build --release

FROM debian:stretch-slim as runtime

RUN apt-get update \
    && apt-get install -y \
        linux-perf
COPY --from=build /app /app

WORKDIR /app
#RUN perf record --call-graph dwarf -- ./urubu
#RUN perf report