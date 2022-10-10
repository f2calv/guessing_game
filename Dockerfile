ARG BASE_IMAGE=rust:1.64.0
ARG FINAL_IMAGE=rust:1.64.0
FROM $BASE_IMAGE AS base
RUN rustup component add clippy
RUN rustup component add rustfmt
#https://github.com/LukeMathWalker/cargo-chef
#RUN cargo install cargo-chef --locked


FROM base AS dependencies
WORKDIR /app/
#initialize empty application
RUN cargo init
#replace the toml dependency file with our own
COPY ./Cargo.toml ./Cargo.lock /app/
#run a build to download the dependencies
RUN cargo build --release


FROM dependencies AS source
COPY ./src/ /app/src/


FROM source AS build
RUN cargo build --release


FROM $FINAL_IMAGE AS final
WORKDIR /app
COPY --from=build /app/target/release/guessing_game /app
ENTRYPOINT ["./app"]