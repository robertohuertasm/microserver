FROM rust:1.40-stretch as builder
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install microserver --version 0.1.6 --target=x86_64-unknown-linux-musl
# Add tini here because chmod doesn't
# exist in the final container
ENV TINI_VERSION v0.18.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini


FROM gcr.io/distroless/base:nonroot@sha256:2b177fbc9a31b85254d264e1fc9a65accc6636d6f1033631b9b086ee589d1fe2
COPY --from=builder /usr/local/cargo/bin/microserver /microserver
COPY --from=builder /tini /tini
EXPOSE 9090
ENTRYPOINT ["/tini", "--"]
CMD ["/microserver", "/app"]
