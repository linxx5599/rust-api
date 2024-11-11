use std::io::Cursor;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::tokio::io::AsyncReadExt;
use rocket::Request;
use rocket::Response;
use serde_json::json;

pub struct JsonResponseFairing;

#[rocket::async_trait]
impl Fairing for JsonResponseFairing {
    fn info(&self) -> Info {
        Info {
            name: "JSON Response Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) -> () {
        match response.body_mut() {
            body => {
                let mut body_bytes: Vec<u8> = Vec::new();
                if let Some(data) = body.into() {
                    data.read_to_end(&mut body_bytes).await.unwrap();
                    let body_str = String::from_utf8(body_bytes).unwrap();
                    let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
                    let json_body = json!({ "data": body_json, "status": Status::Ok.code, "message": "操作成功" });
                    let cursor = Cursor::new(json_body.to_string());
                    response.set_sized_body(cursor.get_ref().len(), cursor);
                } else {
                    // Handle the case where the body is not a Data body
                    let error_message = "Invalid body type";
                    let json_body = json!({
                        "data": null,
                        "status": Status::BadRequest.code,
                        "message": error_message
                    });
                    let cursor = Cursor::new(json_body.to_string());
                    response.set_sized_body(cursor.get_ref().len(), cursor);
                }
            }
        }
    }
}
