FROM ubuntu:22.04
COPY ./target/release/my-swagger-hub ./target/release/my-swagger-hub 
ENTRYPOINT ["./target/release/my-swagger-hub"]