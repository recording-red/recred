# builder image to do the dirty prep stuff
FROM rust:1.65-slim-buster as builder

# install pkg-config
RUN apt update -y
RUN apt upgrade -y
RUN apt install pkg-config openssl libssl-dev -y

# copy source
WORKDIR /app
COPY Cargo.toml .
COPY Cargo.lock .
COPY api api/
COPY core core/
COPY entity entity/
COPY migration migration/
COPY src src/

# build
RUN cargo build --workspace 
RUN cargo install --path . 

# final image to only have the bin
FROM debian:buster-slim
RUN apt update -y

# installs openssl since we use native tls in the actix runtime
# maybe I could switch to rusttls, and skip that
# not sure which one is the most efficient / bug free
RUN apt install openssl -y
COPY --from=builder /usr/local/cargo/bin/recred /usr/local/bin/recred
# a bit ugly, but I'll try to remove the .env to use env var
ADD .env_aws .env

EXPOSE 8080
ENTRYPOINT ["recred"]
