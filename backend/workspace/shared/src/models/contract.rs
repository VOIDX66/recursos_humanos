#[derive(Debug)]
pub struct ContractData {
    pub nombre_completo: String,
    pub id_number: String,
    pub titulo_vacante: String,
    pub salario: Option<f64>,
}

#[derive(serde::Deserialize)]
pub struct ContractQuery {
    pub user_id: String,
    pub vacancy_id: String,
}