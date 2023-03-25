FROM rust:slim
COPY ./target/release/my-swagger-hub ./target/release/my-swagger-hub 
ENTRYPOINT ["./target/release/my-swagger-hub"]