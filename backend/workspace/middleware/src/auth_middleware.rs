use actix_web::{dev::ServiceRequest, Error, HttpMessage, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use shared::models::user::Claims;
use shared::responses::json_response::json_response;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    // Acceder al JWT_SECRET desde el estado
    let jwt_secret = {
        let data = req
            .app_data::<web::Data<shared::state::app_state::AppState>>()
            .expect("AppState missing");
        data.jwt_secret.clone()
    };

    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::default();

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        }
        Err(_) => {
            let resp = json_response("Invalid or expired token", 401);
            let err = actix_web::error::InternalError::from_response("Unauthorized", resp).into();
            Err((err, req))  // 👈 Importante: este tuple es lo que se necesita
        }
    }
}
