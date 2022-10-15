FROM lukemathwalker/cargo-chef:latest-rust-1.62 as planner
WORKDIR /cadency
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1.62 as cacher
WORKDIR /cadency
COPY --from=planner /cadency/recipe.json recipe.json
RUN apt-get update && apt-get install -y cmake && apt-get autoremove -y && \
cargo chef cook --release --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1.62 as builder
WORKDIR /cadency
COPY . .
COPY --from=cacher /cadency/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin cadency

FROM bitnami/minideb:bullseye as packages
# Downloads both ffmpeg and yt-dlp
WORKDIR packages
# tar: (x) extract, (J) from .xz, (f) a file. (--wildcards */bin/ffmpeg) any path with /bin/ffmpeg, (--transform) remove all previous paths
# FFMPEG is staticly compiled, so platform specific
# If statement: converts architecture from docker to a correct link. Default is amd64 = desktop 64 bit
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then \
    export LINK="https://github.com/yt-dlp/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linuxarm64-gpl.tar.xz"; \
  else \
    export LINK="https://github.com/yt-dlp/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz"; \
  fi && \
  apt-get update && apt-get install -y curl tar xz-utils && \
curl -L $LINK > ffmpeg.tar.xz && \
tar -xJf ffmpeg.tar.xz --wildcards */bin/ffmpeg --transform='s/^.*\///' && rm ffmpeg.tar.xz
RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux > yt-dlp && chmod +x yt-dlp

FROM bitnami/minideb:bullseye as python-builder
# Based on: https://github.com/zarmory/docker-python-minimal/blob/master/Dockerfile
# Removes Python build and developmenttools like pip.
RUN apt-get update && apt-get install -y python3-minimal binutils && \
rm -rf /usr/local/lib/python*/ensurepip && \
rm -rf /usr/local/lib/python*/idlelib && \
rm -rf /usr/local/lib/python*/distutils/command && \
rm -rf /usr/local/lib/python*/lib2to2 && \
rm -rf /usr/local/lib/python*/__pycache__/* && \
find /usr/local/bin -not -name 'python*' \( -type f -o -type l \) -exec rm {} \;&& \
rm -rf /usr/local/share/*

FROM bitnami/minideb:bullseye
LABEL org.opencontainers.image.source="https://github.com/jontze/cadency-rs"
WORKDIR /cadency
COPY --from=builder /cadency/target/release/cadency cadency
COPY --from=packages /packages /usr/bin
COPY --from=python-builder /usr/local/ /usr/local/

CMD [ "./cadency" ]
