use actix_web::{web, HttpRequest, HttpMessage, Responder, Result};
use shared::state::app_state::AppState;
use shared::responses::errors::AppError;
use shared::models::contract::ContractQuery;
use shared::models::user::Claims;
use crate::services::contract_services;
use actix_files::NamedFile;
use mime;
use actix_web::http::header::{ContentDisposition, DispositionType};

pub async fn generate_contract_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    query: web::Json<ContractQuery>,
) -> Result<impl Responder, AppError> {
    // ✅ 1. Extraer claims del request
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("No se encontraron los claims del usuario".to_string()))?;

    // ✅ 2. Verificar rol autorizado
    let allowed_roles = ["admin", "evaluador"];
    if !allowed_roles.contains(&claims.role.as_str()) {
        return Err(AppError::Unauthorized("No tiene permisos para generar contratos".to_string()));
    }

    // ✅ 3. Obtener conexión a la base de datos
    let client = app_state.pool.get().await
        .map_err(|e| AppError::InternalServerError(format!("Error de conexión a BD: {}", e)))?;

    // ✅ 4. Obtener datos del contrato
    let contract_data = contract_services::get_contract_data(
        &client,
        &query.user_id,
        &query.vacancy_id,
    ).await?;

    // ✅ 5. Definir rutas estáticas
    let fuente_path = Some("assets/fonts/FuentePrincipal.ttf");
    let firma_path = Some("workspace/utils/genPdf/firma.png");
    let salida_pdf_path = format!("docs/contracts/contrato_{}.pdf", query.user_id);

    // ✅ 6. Ejecutar binario para generar el contrato
    contract_services::run_contract_script(
        &contract_data,
        fuente_path,
        firma_path,
        &salida_pdf_path,
    ).await?;

    // ✅ 7. Respuesta
    let named_file = NamedFile::open_async(&salida_pdf_path)
        .await
        .map_err(|e| AppError::InternalServerError(format!("No se pudo abrir el PDF generado: {}", e)))?;

    Ok(named_file
        .use_last_modified(true)
        .set_content_type(mime::APPLICATION_PDF)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        })
        .into_response(&req))
}