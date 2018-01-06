FROM rustlang/rust:nightly

RUN cargo install -j4 diesel_cli --no-default-features --features postgres

RUN cargo install -j4 cargo-watch

WORKDIR /usr/src/app

EXPOSE 8080

VOLUME ["/usr/local/cargo"]
