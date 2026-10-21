# The first stage builds the binary, and the second copies it into a small
# image. The browser runtime is compiled into the binary.
FROM rust:1.99-slim-trixie AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:trixie-slim
COPY --from=build /app/target/release/my-app /usr/local/bin/my-app
ENV PORT=3000
EXPOSE 3000
CMD ["my-app"]
