# 拉取rust编译环境
FROM  m.daocloud.io/docker.io/rust:latest

# 定义环境变量
ENV CLUSTER_PATH="src/k3s.yaml"

# 设置工作目录
WORKDIR /rc

# 复制Cargo.toml和Cargo.lock文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src

RUN chmod +x /usr/local/rustup/toolchains/1.82.0-x86_64-unknown-linux-gnu/bin/rustc

# 构建项目
RUN cargo build --release

# 设置工作目录
WORKDIR /rc/target/release

# 运行项目
CMD ["./rc"]