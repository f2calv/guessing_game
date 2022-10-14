ARG BASE_IMAGE=rust:1.64.0
ARG FINAL_IMAGE=rust:1.64.0
FROM $BASE_IMAGE AS base
RUN apt-get update && apt-get install -yq lld clang
RUN rustup component add clippy
RUN rustup component add rustfmt


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
RUN apt-get update && apt-get install -yq lld clang
COPY --from=build /app/target/release/rust-playground rust-playground

ARG GIT_REPO
ENV GIT_REPO=$GIT_REPO
ARG GIT_TAG
ENV GIT_TAG=$GIT_TAG
ARG GIT_BRANCH
ENV GIT_BRANCH=$GIT_BRANCH
ARG GIT_COMMIT
ENV GIT_COMMIT=$GIT_COMMIT

ARG GITHUB_WORKFLOW
ENV GITHUB_WORKFLOW=$GITHUB_WORKFLOW
ARG GITHUB_RUN_ID
ENV GITHUB_RUN_ID=$GITHUB_RUN_ID
ARG GITHUB_RUN_NUMBER
ENV GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER

EXPOSE 8

ENTRYPOINT ["./rust-playground"]