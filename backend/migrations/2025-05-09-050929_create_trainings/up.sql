-- Your SQL goes here
CREATE TABLE trainings (
    id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id VARCHAR NOT NULL,                          -- El empleado asignado a la capacitación
    trainer_id VARCHAR,                                    -- El capacitador (puede ser nulo si no es necesario)
    training_type VARCHAR(50) NOT NULL,                 -- Tipo de capacitación (por ejemplo, "2 meses", "anual")
    training_date TIMESTAMP WITH TIME ZONE NOT NULL,    -- Fecha de la capacitación
    feedback TEXT,                                      -- Retroalimentación del capacitador
    status VARCHAR(20) NOT NULL DEFAULT 'pending',       -- Estado de la capacitación: pending, completed
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_employee FOREIGN KEY (employee_id) REFERENCES users(user_id) ON DELETE CASCADE,
    CONSTRAINT fk_trainer FOREIGN KEY (trainer_id) REFERENCES users(user_id) ON DELETE SET NULL
);
