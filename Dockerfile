FROM rust:1.67

WORKDIR /usr/src/echo_service
COPY echo_service/ .

RUN rustup component add rustfmt
RUN cargo install --path .
EXPOSE 50051
# CMD ["/usr/src/myapp/target/release/server"]
# COPY /usr/src/echo_service/target/release/server . 
CMD ["server"]