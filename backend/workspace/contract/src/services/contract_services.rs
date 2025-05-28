use tokio_postgres::Client;
use tokio::process::Command;
use shared::responses::errors::AppError;
use shared::models::contract::ContractData;
use std::env;
use std::path::PathBuf;

pub async fn get_contract_data(conn: &Client, user_id: &str, vacancy_id: &str) -> Result<ContractData, AppError> {
    // Preparar consultas
    let user_stmt = conn
        .prepare(
            "SELECT name, lastname, id_number
             FROM users
             WHERE user_id = $1",
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to prepare user query: {}", e)))?;

    let vacancy_stmt = conn
        .prepare(
            "SELECT title, salary
             FROM vacancies
             WHERE id = $1",
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to prepare vacancy query: {}", e)))?;

    // Ejecutar consulta usuario
    let user_row = conn
        .query_opt(&user_stmt, &[&user_id])
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to execute user query: {}", e)))?
        .ok_or_else(|| AppError::NotFoundError(format!("User with id {} not found", user_id)))?;

    // Ejecutar consulta vacante
    let vacancy_row = conn
        .query_opt(&vacancy_stmt, &[&vacancy_id])
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to execute vacancy query: {}", e)))?
        .ok_or_else(|| AppError::NotFoundError(format!("Vacancy with id {} not found", vacancy_id)))?;

    // Extraer campos
    let name: String = user_row.get("name");
    let lastname: String = user_row.get("lastname");
    let id_number: String = user_row.get("id_number");

    let title: String = vacancy_row.get("title");
    let salary: Option<f64> = vacancy_row.get("salary");

    let nombre_completo = format!("{} {}", name, lastname);

    // Actualizar estado de la vacante a 'closed'
    conn.execute(
        "UPDATE vacancies SET status = 'closed' WHERE id = $1",
        &[&vacancy_id],
    )
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to update vacancy status: {}", e)))?;

    Ok(ContractData {
        nombre_completo,
        id_number,
        titulo_vacante: title,
        salario: salary,
    })
}

pub async fn run_contract_script(
    contract: &ContractData,
    fuente_path: Option<&str>,
    firma_path: Option<&str>,
    salida_pdf_path: &str,
) -> Result<(), AppError> {
    let salario_str = contract.salario
        .map(|s| s.to_string())
        .unwrap_or_else(|| "".to_string());

    let exe_path = env::current_exe()
    .map_err(|e| AppError::InternalServerError(format!("No se pudo obtener el path del ejecutable: {}", e)))?;

    let mut bin_path = PathBuf::from(exe_path);
    bin_path.pop(); 
    bin_path.pop(); 
    bin_path.pop(); 
    bin_path.push("workspace/utils/genPdf/dist/genContractPdf");
    let mut cmd = Command::new(bin_path);
    //println!("Ejecutando binario: {:?}", cmd);

    cmd.arg("--titulo").arg(&contract.titulo_vacante)
        .arg("--salario").arg(&salario_str)
        .arg("--nombre").arg(&contract.nombre_completo)
        .arg("--id").arg(&contract.id_number)
        .arg("--salida").arg(&salida_pdf_path);

    if let Some(fuente) = fuente_path {
        cmd.arg("--fuente").arg(fuente);
    }

    if let Some(firma) = firma_path {
        cmd.arg("--firma").arg(firma);
    }

    let output = cmd.output()
        .await
        .map_err(|e| AppError::DatabaseError(format!("Error ejecutando script python: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::DatabaseError(format!("Error en script python: {}", stderr)));
    }

    Ok(())
}