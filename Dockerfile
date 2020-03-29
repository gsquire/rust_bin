FROM debian:latest AS build
RUN apt-get update && apt-get install -y build-essential wget
RUN wget -q https://static.rust-lang.org/dist/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
RUN tar xf rust-nightly-x86_64-unknown-linux-gnu.tar.gz
WORKDIR rust-nightly-x86_64-unknown-linux-gnu
RUN ./install.sh

WORKDIR /build
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=build /build/target/release/rust_bin /
COPY --from=build /build/routes.html /
ENV ROCKET_CLI_COLORS=off
CMD ["/rust_bin"]
