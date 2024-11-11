## Rust编译器和工具链

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

```bash
rustup update stable
```

## start

```bash
cargo run
```

## build

```bash
cargo build
```

## check

```bash
cargo check
```

## Rocket 基础配置 Rocket.toml

```bash
# Rocket默认配置
[default]
address = "127.0.0.1"  # 服务器监听的IP地址
port = 8000  # 服务器监听的端口号

# 生产环境配置
[production]
address = "0.0.0.0"  # 生产环境中监听所有网络接口
port = 80  # 使用80端口以处理HTTP请求
```

### 基本使用

```rust
use rocket::Route;
use serde_json::{json, Value};

#[path = "./node_service.rs"]
mod node_service;

#[get("/node")]
fn get_node() -> Value {
    let result = node_service::get_node();
    json!(result)
}

#[post("/node")]
fn create_node() -> &'static str {
    "Hello, node!"
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_node, create_node]
}

```
