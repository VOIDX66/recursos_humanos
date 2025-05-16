-- Your SQL goes here
CREATE TABLE vacancies (
    id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    requirements TEXT,
    salary DOUBLE PRECISION,
    opening_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_DATE,
    closing_date TIMESTAMPTZ,
    status VARCHAR(20) NOT NULL DEFAULT 'open',
    created_by VARCHAR NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_DATE,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_DATE,

    CONSTRAINT fk_created_by FOREIGN KEY (created_by) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Búsquedas frecuentes por estado de la vacante (ej. 'open', 'closed')
CREATE INDEX idx_vacancies_status ON vacancies(status);

-- Para buscar vacantes por analista/creador
CREATE INDEX idx_vacancies_created_by ON vacancies(created_by);

-- Para filtrar vacantes abiertas por fecha de cierre
CREATE INDEX idx_vacancies_closing_date ON vacancies(closing_date);

-- Para ordenar o buscar por salario
CREATE INDEX idx_vacancies_salary ON vacancies(salary);
