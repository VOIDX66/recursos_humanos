-- Your SQL goes here
CREATE TABLE vacancies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    requirements TEXT,
    salary NUMERIC(10, 2),
    opening_date DATE NOT NULL DEFAULT CURRENT_DATE,
    closing_date DATE,
    status VARCHAR(20) NOT NULL DEFAULT 'open',
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

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
