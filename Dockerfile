FROM lukemathwalker/cargo-chef:latest-rust-1.61.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim as runner
RUN apt-get update -y
RUN apt remove mysql-server
RUN apt autoremove
RUN apt-get remove --purge mysql\*
RUN apt-get install wget -y
# Required to install mysql
# libmysqlclient-dev necessary for diesel's mysql integration
RUN apt-get install -y default-libmysqlclient-dev
# Add Oracle MySQL repository
RUN apt-get update
RUN apt-get install -y gnupg lsb-release wget
RUN wget https://dev.mysql.com/get/mysql-apt-config_0.8.22-1_all.deb
RUN DEBIAN_FRONTEND=noninteractive dpkg -i mysql-apt-config_0.8.22-1_all.deb
RUN apt update
# Add Oracle's libmysqlclient-dev
RUN apt-get install -y libmysqlclient-dev mysql-community-client
# Copy demo-api-svc executable
COPY --from=builder /usr/local/cargo/bin/demo-api-svc /usr/local/bin/demo-api-svc
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["demo-api-svc"]
