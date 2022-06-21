FROM rust:slim
COPY ./target/release/my_swagger_hub ./target/release/my_swagger_hub 
ENTRYPOINT ["./target/release/my_swagger_hub"]