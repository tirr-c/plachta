FROM rust:1
WORKDIR /usr/src/plachta
COPY . .
RUN cargo build --release --bin webserver

FROM bitnami/minideb:stretch
RUN install_packages libpq-dev
COPY --from=0 /usr/src/plachta/target/release/webserver /plachta

EXPOSE 8080
ENTRYPOINT ["/plachta"]
