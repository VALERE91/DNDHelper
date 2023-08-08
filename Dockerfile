FROM rust:1.71 as build

# create a new empty shell project
RUN USER=root cargo new --bin dnd_helper
WORKDIR /dnd_helper

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./logconfig.yml ./logconfig.yml

# build for release
RUN rm ./target/release/deps/dnd_helper*
RUN cargo build --release

# our final base
FROM debian:bullseye

# copy the build artifact from the build stage
COPY --from=build /dnd_helper/target/release/dnd_helper_api .
COPY --from=build /dnd_helper/logconfig.yml .

# set the startup command to run your binary
CMD ["./dnd_helper_api"]