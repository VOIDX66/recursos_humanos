CREATE TABLE IF NOT EXISTS trainings (
    id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id VARCHAR NOT NULL,                          -- El empleado asignado a la capacitación
    trainer_id VARCHAR,                                    -- El capacitador (puede ser nulo si no es necesario)
    training_type VARCHAR(50) NOT NULL,                 -- Tipo de capacitación (por ejemplo, "2 meses", "anual")
    training_date TIMESTAMPTZ NOT NULL,    -- Fecha de la capacitación
    feedback TEXT,                                      -- Retroalimentación del capacitador
    status VARCHAR(20) NOT NULL DEFAULT 'pending',       -- Estado de la capacitación: pending, completed
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_employee FOREIGN KEY (employee_id) REFERENCES users(user_id) ON DELETE CASCADE,
    CONSTRAINT fk_trainer FOREIGN KEY (trainer_id) REFERENCES users(user_id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_trainings_employee_id ON trainings (employee_id);
CREATE INDEX IF NOT EXISTS idx_trainings_trainer_id ON trainings (trainer_id);
CREATE INDEX IF NOT EXISTS idx_trainings_status ON trainings (status);
CREATE INDEX IF NOT EXISTS idx_trainings_training_date ON trainings (training_date);
