use rocket::Route;
use serde_json::{json, Value};

#[path = "./host_service.rs"]
mod host_service;

#[get("/host")]
async fn get_host() -> Value {
    let result= host_service::get_host().await;
    json!(result)
}

#[post("/host")]
fn create_host() -> &'static str {
    "Hello, host!"
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_host, create_host]
}
