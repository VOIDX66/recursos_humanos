-- Your SQL goes here
CREATE TABLE evaluations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vacancy_id UUID NOT NULL,                           -- La vacante asociada
    candidate_id UUID NOT NULL,                         -- El postulante evaluado
    evaluator_id UUID NOT NULL,                         -- El evaluador que realiza la evaluación
    evaluation_date TIMESTAMP WITH TIME ZONE NOT NULL,  -- Fecha de la evaluación
    feedback TEXT,                                     -- Retroalimentación del evaluador
    score NUMERIC(5, 2),                               -- Calificación de la evaluación (opcional)
    status VARCHAR(20) NOT NULL DEFAULT 'pending',      -- Estado de la evaluación: pending, completed, etc.
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fk_vacancy FOREIGN KEY (vacancy_id) REFERENCES vacancies(id) ON DELETE CASCADE,
    CONSTRAINT fk_candidate FOREIGN KEY (candidate_id) REFERENCES users(user_id) ON DELETE CASCADE,
    CONSTRAINT fk_evaluator FOREIGN KEY (evaluator_id) REFERENCES users(user_id) ON DELETE CASCADE
);
