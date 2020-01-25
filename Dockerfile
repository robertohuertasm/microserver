FROM ekidd/rust-musl-builder AS builder
# Add our source code.
ADD . ./
# build for musl
RUN cargo build --release
# Add tini here because chmod doesn't exist in the final container
ENV TINI_VERSION v0.18.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN sudo chown -R rust:rust /tini
RUN chmod +x /tini


FROM gcr.io/distroless/base:nonroot@sha256:2b177fbc9a31b85254d264e1fc9a65accc6636d6f1033631b9b086ee589d1fe2
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/microserver /
COPY --from=builder /tini /tini
EXPOSE 9090
ENTRYPOINT ["/tini", "--"]
CMD ["/microserver", "/app"]
