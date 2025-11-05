FROM rust:1.91.0

RUN apt-get update && apt-get install -y \
    tig

ARG USER
ARG UID
RUN useradd -m -s /bin/bash -u ${UID:-2222} $USER
USER ${USER}

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/download/v0.30.0/probe-rs-tools-installer.sh | sh

RUN rustup target add thumbv7em-none-eabihf

WORKDIR /home/$USER/dependencies_fetch_project/dummy/stm32f446re_example
RUN cargo init
COPY ./stm32f446re_example/Cargo.toml .
RUN cargo fetch

WORKDIR /home/$USER/dependencies_fetch_project/dummy/std_example
RUN cargo init
COPY ./std_example/Cargo.toml .
RUN cargo fetch

# - For example when used as devcontainer, the UID is set to a default value (see above).
#   I wasn't able to pass the UID of the local user to the container in this case.
#   So when using the devcontainer, the local user is then used and can't access the fetched dependencies.
#   To solve this, the fetched dependencies are made readable and writable by anyone.
RUN chmod -R a+rw /usr/local/cargo/registry
