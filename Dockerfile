FROM ubuntu:latest
COPY target/aarch64-unknown-linux-gnu/release/s3-server /s3-server
EXPOSE 8014
WORKDIR /
CMD ["./s3-server"]