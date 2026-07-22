FROM mcr.microsoft.com/devcontainers/rust:2.0.13-1-trixie

RUN apt-get update && apt-get install -y \
    tig

USER vscode

RUN rustup target add thumbv7em-none-eabihf

RUN cargo install --locked probe-rs-tools@0.31.0

WORKDIR /home/vscode/dependencies_fetch_project/dummy/stm32f446re_example
RUN cargo init
COPY ../stm32f446re_example/Cargo.toml .
RUN cargo fetch

WORKDIR /home/vscode/dependencies_fetch_project/dummy/stm32g474re_example
RUN cargo init
COPY ../stm32g474re_example/Cargo.toml .
RUN cargo fetch

WORKDIR /home/vscode/dependencies_fetch_project/dummy/std_example
RUN cargo init
COPY ../std_example/Cargo.toml .
RUN cargo fetch
