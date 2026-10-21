# The first stage builds the binary, and the second copies it into a small
# image. The browser runtime is compiled into the binary.
FROM rust:1.99-slim-trixie AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:trixie-slim
COPY --from=build /app/target/release/{{project-name}} /usr/local/bin/{{project-name}}
ENV PORT=3000
EXPOSE 3000
CMD ["{{project-name}}"]
