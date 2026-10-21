# The first stage builds the binary, and the second copies it into a small
# image. The browser runtime is compiled into the binary.
FROM rust:1.99-slim-trixie AS build
WORKDIR /app
COPY . .
# Renaming the one binary the package installs lets this file name no package,
# so it builds any single-binary project as it is. A package with more than
# one binary stops the build here, because `mv` takes one source.
RUN cargo install --path . --root /out && mv /out/bin/* /out/server

FROM debian:trixie-slim
COPY --from=build /out/server /usr/local/bin/server
ENV PORT=3000
EXPOSE 3000
CMD ["server"]
