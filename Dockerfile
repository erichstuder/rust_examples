FROM rust:1.86.0

RUN apt-get update && apt-get install -y \
    tig

ARG USER
ARG UID
RUN useradd -m -s /bin/bash -u ${UID:-2222} $USER
USER ${USER}
