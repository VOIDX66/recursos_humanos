use actix_web::HttpResponse;
use crate::responses::json_response::json_response; // Importa la función json_response

pub async fn welcome() -> HttpResponse {
    // Usamos la función json_response para devolver la respuesta
    json_response("¡Bienvenido a la aplicación de Recursos Humanos!", 200)
}
