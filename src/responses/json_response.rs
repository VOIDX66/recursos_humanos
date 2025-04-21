use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

pub fn json_response(message: &str, status_code: u16) -> HttpResponse {
    let response = ApiResponse {
        message: message.to_string(),
    };
    HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
        .json(response)
}
