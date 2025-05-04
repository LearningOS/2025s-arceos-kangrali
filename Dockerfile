FROM ubuntu:latest

RUN apt update && apt install -y \
    wget \
    xxd \
    curl \
    gcc \
    g++ \
    make \
    libclang-dev \
    qemu-system-misc \
    bash \
    sudo \
    git \
    vim \
    dosfstools \
    build-essential \
    pkg-config \
    libssl-dev \
    libz-dev \
    libclang-dev && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

# 安装 Rust 和 cargo-binutils
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static \
    RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN echo "[source.crates-io]" > /root/.cargo/config.toml && \
    echo "replace-with = 'ustc'" >> /root/.cargo/config.toml && \
    echo "\n" >> /root/.cargo/config.toml && \
    echo "[source.ustc]" >> /root/.cargo/config.toml && \
    echo "registry = 'sparse+https://mirrors.ustc.edu.cn/crates.io-index/'" >> /root/.cargo/config.toml

RUN cargo install cargo-binutils

# 安装 musl toolchains
WORKDIR /opt/musl
RUN wget https://musl.cc/aarch64-linux-musl-cross.tgz && \
    wget https://musl.cc/riscv64-linux-musl-cross.tgz && \
    wget https://musl.cc/x86_64-linux-musl-cross.tgz && \
    tar zxf aarch64-linux-musl-cross.tgz && \
    tar zxf riscv64-linux-musl-cross.tgz && \
    tar zxf x86_64-linux-musl-cross.tgz

# 添加 musl 工具链路径到环境变量
ENV PATH="/opt/musl/x86_64-linux-musl-cross/bin:/opt/musl/aarch64-linux-musl-cross/bin:/opt/musl/riscv64-linux-musl-cross/bin:${PATH}"

# 设置工作目录
WORKDIR /mnt/
