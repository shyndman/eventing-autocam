# syntax=docker/dockerfile:1
ARG DOCKER_REPO
FROM --platform=$BUILDPLATFORM $DOCKER_REPO/balenalib/amd64-debian:bullseye-build AS machine_base
WORKDIR /root

# Update the package manager and install some basics
RUN --mount=type=cache,target=/var/cache/apt,sharing=shared \
    --mount=type=cache,target=/var/lib/apt,sharing=shared \
    apt-get update && \
    apt-get --assume-yes --no-install-recommends install \
    libssl-dev ca-certificates gnupg lsb-release curl wget bash

# ~~~~ ZSH SHELL SETUP
FROM machine_base AS shell_setup
COPY include/install_shell.sh install_shell.sh
COPY include/.p10k.zsh .p10k.zsh
RUN --mount=type=cache,target=/var/cache/apt,sharing=shared \
    --mount=type=cache,target=/var/lib/apt,sharing=shared \
    chmod +x install_shell.sh && \
    ./install_shell.sh \
        -t 'https://github.com/romkatv/powerlevel10k' \
        -p 'https://github.com/zsh-users/zsh-syntax-highlighting.git' \
        -a '[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh'

# ~~~~ RUST
FROM shell_setup AS rustup_builder
RUN curl https://sh.rustup.rs -sSf > rust_init.sh; \
    chmod +x rust_init.sh;
RUN ["/bin/zsh", "-c", "\
    ./rust_init.sh -y \
          --default-host=x86_64-unknown-linux-gnu \
          --default-toolchain=nightly; \
    source '.cargo/env'; \
"]