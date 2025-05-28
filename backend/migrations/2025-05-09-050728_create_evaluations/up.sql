-- Your SQL goes here
CREATE TABLE IF NOT EXISTS evaluations (
    id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid(),
    vacancy_id VARCHAR NOT NULL,                           -- La vacante asociada
    candidate_id VARCHAR NOT NULL,                         -- El postulante evaluado
    evaluator_id VARCHAR NOT NULL,                         -- El evaluador que realiza la evaluación
    evaluation_date TIMESTAMPTZ NOT NULL,  -- Fecha de la evaluación
    feedback TEXT,                                     -- Retroalimentación del evaluador
    score DOUBLE PRECISION,                               -- Calificación de la evaluación (opcional)
    status VARCHAR(20) NOT NULL DEFAULT 'pending',      -- Estado de la evaluación: pending, completed, etc.
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fk_vacancy FOREIGN KEY (vacancy_id) REFERENCES vacancies(id) ON DELETE CASCADE,
    CONSTRAINT fk_candidate FOREIGN KEY (candidate_id) REFERENCES users(user_id) ON DELETE CASCADE,
    CONSTRAINT fk_evaluator FOREIGN KEY (evaluator_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Índices para acelerar búsquedas y joins comunes
CREATE INDEX IF NOT EXISTS idx_evaluations_vacancy_id ON evaluations (vacancy_id);
CREATE INDEX IF NOT EXISTS idx_evaluations_candidate_id ON evaluations (candidate_id);
CREATE INDEX IF NOT EXISTS idx_evaluations_evaluator_id ON evaluations (evaluator_id);
CREATE INDEX IF NOT EXISTS idx_evaluations_status ON evaluations (status);
CREATE INDEX IF NOT EXISTS idx_evaluations_evaluation_date ON evaluations (evaluation_date);
