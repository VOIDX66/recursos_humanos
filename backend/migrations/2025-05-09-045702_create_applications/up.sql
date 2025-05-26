-- Your SQL goes here
-- migrations/sql/up.sql

CREATE TABLE applications (
    id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid(),
    
    user_id VARCHAR NOT NULL,
    vacancy_id VARCHAR NOT NULL,
    
    application_date TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- Ej: pending, reviewed, accepted, rejected
    comment TEXT,

    updated_at TIMESTAMPTZ DEFAULT CURRENT_DATE,

    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    CONSTRAINT fk_vacancy FOREIGN KEY (vacancy_id) REFERENCES vacancies(id) ON DELETE CASCADE,
    
    CONSTRAINT unique_application UNIQUE (user_id, vacancy_id)
);

-- Índices útiles
CREATE INDEX idx_applications_status ON applications(status);
CREATE INDEX idx_applications_user ON applications(user_id);
CREATE INDEX idx_applications_vacancy ON applications(vacancy_id);
