FROM rustlang/rust:nightly

# Want to cache dependencies eventually:
# https://github.com/rust-lang/cargo/issues/2644

RUN cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 8000

VOLUME ["/usr/local/cargo"]
