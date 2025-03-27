// responses.rs

use actix_web::HttpResponse;  // Eliminar `Responder` ya que no es necesario
use serde::Serialize;  // Mantener este import

// Definimos una estructura para la respuesta
#[derive(Serialize)]  // Agrega este atributo para derivar `Serialize`
pub struct ApiResponse {
    pub message: String,
}

// Función que maneja respuestas JSON con código de estado
pub fn json_response(message: &str, status_code: u16) -> HttpResponse {
    let response = ApiResponse {
        message: message.to_string(),
    };
    HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
        .json(response)
}
