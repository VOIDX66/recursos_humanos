use crate::responses::errors::AppError;
use crate::models::vacancy::{NewVacancy, Vacancy};
use chrono::{DateTime, Utc};
use deadpool_postgres::Client;

pub async fn create_vacancy(
    conn: &Client,
    new_vacancy: NewVacancy,
    created_by: &str,
) -> Result<Vacancy, AppError> {
    let stmt = conn
        .prepare(
            "INSERT INTO vacancies 
             (title, description, requirements, salary, opening_date, closing_date, status, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, 'open', $7, $8, $8)
             RETURNING id, title, description, requirements, salary, opening_date, closing_date, status, created_by, created_at, updated_at",
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to prepare vacancy insert query: {}", e)))?;

    let now: DateTime<Utc> = Utc::now();

    // Aquí abrimos y cerramos con DateTime<Utc>, no NaiveDateTime
    let opening_datetime = now;
    let closing_datetime = new_vacancy.closing_date.map(|dt| {
        // Suponiendo que new_vacancy.closing_date es DateTime<Utc>
        dt
    });

    let rows = conn
        .query(
            &stmt,
            &[
                &new_vacancy.title,
                &new_vacancy.description,
                &new_vacancy.requirements,
                &new_vacancy.salary,
                &opening_datetime,
                &closing_datetime,
                &created_by,
                &opening_datetime, // created_at y updated_at usan la misma fecha/hora
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to execute vacancy insert query: {}", e)))?;

    if let Some(row) = rows.get(0) {
        Ok(Vacancy {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            requirements: row.get("requirements"),
            salary: row.get("salary"),
            opening_date: row.get::<_, DateTime<Utc>>("opening_date"),
            closing_date: row.get::<_, Option<DateTime<Utc>>>("closing_date"),
            status: row.get("status"),
            created_by: row.get("created_by"),
            created_at: row.get::<_, DateTime<Utc>>("created_at"),
            updated_at: row.get::<_, DateTime<Utc>>("updated_at"),
        })
    } else {
        Err(AppError::DatabaseError("Failed to insert vacancy".into()))
    }
}

pub async fn get_all_vacancies(client: &Client) -> Result<Vec<Vacancy>, AppError> {
    let stmt = client
        .prepare("SELECT id, title, description, requirements, salary, opening_date, closing_date, status, created_by, created_at, updated_at FROM vacancies")
        .await?;

    let rows = client.query(&stmt, &[]).await?;

    let vacancies = rows
        .iter()
        .map(|row| Vacancy {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            requirements: row.get::<_, Option<String>>("requirements"),
            salary: row.get::<_, Option<f64>>("salary"),
            opening_date: row.get::<_, DateTime<Utc>>("opening_date"),
            closing_date: row.get::<_, Option<DateTime<Utc>>>("closing_date"),
            status: row.get("status"),
            created_by: row.get("created_by"),
            created_at: row.get::<_, DateTime<Utc>>("created_at"),
            updated_at: row.get::<_, DateTime<Utc>>("updated_at"),
        })
        .collect();

    Ok(vacancies)
}