FROM lukemathwalker/cargo-chef:latest-rust-1.62 as planner
WORKDIR /cadency
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1.62 as cacher
WORKDIR /cadency
COPY --from=planner /cadency/recipe.json recipe.json
RUN apt-get update && apt-get install -y cmake 
RUN cargo chef cook --release --recipe-path recipe.json


FROM lukemathwalker/cargo-chef:latest-rust-1.62 as builder
WORKDIR /cadency
COPY . .
COPY --from=cacher /cadency/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin cadency

FROM debian:bullseye-slim
LABEL org.opencontainers.image.source="https://github.com/jontze/cadency-rs"
WORKDIR /cadency
COPY --from=builder /cadency/target/release/cadency cadency
RUN apt-get update && apt-get install -y libopus-dev ffmpeg wget python3
RUN wget https://github.com/yt-dlp/yt-dlp/releases/download/2022.09.01/yt-dlp && chmod +x yt-dlp && mv yt-dlp /usr/bin
CMD [ "./cadency" ]
