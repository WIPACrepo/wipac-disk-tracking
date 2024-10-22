# Dockerfile

#----------------------------------------------------------------------------
# Build the application
#----------------------------------------------------------------------------
FROM rust:slim-bookworm AS build

RUN apt-get update && apt-get install -y libssl-dev pkg-config build-essential

COPY Cargo.lock Cargo.toml /build/
COPY src /build/src

WORKDIR /build
RUN cargo build --release

#----------------------------------------------------------------------------
# Create the application image
#----------------------------------------------------------------------------
FROM debian:bookworm-slim AS image

COPY README.md /
COPY --from=build /build/target/release/wipac-disk-tracking /app/

RUN useradd -m -U app
USER app

WORKDIR /app
CMD [ "/app/wipac-disk-tracking" ]
