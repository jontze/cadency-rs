FROM lukemathwalker/cargo-chef:latest-rust-1.75-slim-bullseye as build_base

FROM build_base as planner
WORKDIR /cadency
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM build_base as cacher
WORKDIR /cadency
COPY --from=planner /cadency/recipe.json recipe.json
ENV RUSTUP_MAX_RETRIES=100
ENV CARGO_INCREMENTAL=0
ENV CARGO_NET_RETRY=100
ENV CARGO_TERM_COLOR=always
RUN apt-get update && apt-get install -y cmake && apt-get autoremove -y
# Build dependencies - this is the dependencies caching layer
RUN cargo chef cook --release --recipe-path recipe.json 

FROM build_base as builder
WORKDIR /cadency
COPY . .
COPY --from=cacher /cadency/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
ENV RUSTUP_MAX_RETRIES=100
ENV CARGO_INCREMENTAL=0
ENV CARGO_NET_RETRY=100
ENV CARGO_TERM_COLOR=always
# Build and cache only the cadency app with the previously builded dependencies
RUN cargo build --release --bin cadency

# Downloads yt-dlp
FROM bitnami/minideb:bullseye as packages
WORKDIR /packages
COPY --from=builder /cadency/.yt-dlprc .
RUN YTDLP_VERSION=$(cat .yt-dlprc) && \
  apt-get update && apt-get install -y curl && \
  curl -L https://github.com/yt-dlp/yt-dlp/releases/download/$YTDLP_VERSION/yt-dlp_linux > yt-dlp && chmod +x yt-dlp

# Based on: https://github.com/zarmory/docker-python-minimal/blob/master/Dockerfile
# Removes Python build and developmenttools like pip.
FROM bitnami/minideb:bullseye as python-builder
RUN apt-get update && apt-get install -y python3-minimal binutils && \
  rm -rf /usr/local/lib/python*/ensurepip && \
  rm -rf /usr/local/lib/python*/idlelib && \
  rm -rf /usr/local/lib/python*/distutils/command && \
  rm -rf /usr/local/lib/python*/lib2to2 && \
  rm -rf /usr/local/lib/python*/__pycache__/* && \
  find /usr/local/bin -not -name 'python*' \( -type f -o -type l \) -exec rm {} \;&& \
  rm -rf /usr/local/share/*

FROM bitnami/minideb:bullseye as runtime
LABEL org.opencontainers.image.source="https://github.com/jontze/cadency-rs"
WORKDIR /cadency
COPY --from=builder /cadency/target/release/cadency cadency
COPY --from=packages /packages /usr/bin
COPY --from=python-builder /usr/local/ /usr/local/
CMD [ "./cadency" ]
