#[macro_use]
extern crate rocket; // 引入Rocket宏

#[path = "./node/node_controller.rs"]
mod node_controller;
#[path = "./pod/pod_controller.rs"]
mod pod_controller;

use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

#[path = "./mod.rs"]
mod json_response_fairing;
 
// 启动Rocket服务器并挂载路由
#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    // 设置CORS配置 attach(cors)
    let allowed_origins = AllowedOrigins::all();
    let cors = CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::all(),
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS");
    rocket::build()
        .attach(cors)
        .attach(json_response_fairing::JsonResponseFairing)
        .mount("/", node_controller::routes())
        .mount("/", pod_controller::routes())
        
}
