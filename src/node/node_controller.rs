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
