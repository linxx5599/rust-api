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
                match body.into() {
                    Some(data) => {
                        data.read_to_end(&mut body_bytes).await.unwrap();
                        let body_str = String::from_utf8(body_bytes).unwrap();
                        let mut _response =
                            json!({"status": Status::Ok.code ,"message": "操作成功" });
                        let error_responses = [
                            (
                                "400: Bad Request",
                                json!({ "status": Status::BadRequest.code, "message": body_str }),
                            ),
                            (
                                "401: Unauthorized",
                                json!({ "status": Status::Unauthorized.code, "message": body_str }),
                            ),
                            (
                                "403: Forbidden",
                                json!({ "status": Status::Forbidden.code, "message": body_str }),
                            ),
                            (
                                "404: Not Found",
                                json!({ "status": Status::NotFound.code, "message": "Not Found" }),
                            ),
                            (
                                "405: Method Not Allowed",
                                json!({ "status": Status::MethodNotAllowed.code, "message": body_str }),
                            ),
                            (
                                "406: Not Acceptable",
                                json!({ "status": Status::NotAcceptable.code, "message": body_str }),
                            ),
                            (
                                "407: Proxy Authentication Required",
                                json!({ "status": Status::ProxyAuthenticationRequired.code, "message": body_str }),
                            ),
                            (
                                "408: Request Timeout",
                                json!({ "status": Status::RequestTimeout.code, "message": body_str }),
                            ),
                            (
                                "500: Internal Server Error",
                                json!({ "status": Status::InternalServerError.code, "message": "Internal Server Error" }),
                            ),
                        ];
                        for (error, error_response) in error_responses {
                            if body_str.contains(error) {
                                _response = error_response;
                                break;
                            }
                        }
                        let status_code = _response["status"].as_u64().unwrap() as u16;
                        if status_code != Status::Ok.code {
                            response.set_status(Status::from_code(status_code).unwrap());
                        } else {
                            let body_json: serde_json::Value =
                                serde_json::from_str(&body_str).unwrap();
                            //给_response添加data字段
                            _response["data"] = body_json;
                        }
                        let cursor = Cursor::new(_response.to_string());
                        response.set_sized_body(cursor.get_ref().len(), cursor);
                    }
                    None => {
                        let json_body = json!({ "status": Status::InternalServerError.code, "message": "Internal Server Error" });
                        response.set_status(Status::from_code(Status::InternalServerError.code).unwrap());
                        let cursor = Cursor::new(json_body.to_string());
                        response.set_sized_body(cursor.get_ref().len(), cursor);
                    }
                }
            }
        }
    }
}
