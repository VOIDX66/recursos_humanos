-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notifications (
    id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR NOT NULL,            -- A quién va dirigida la notificación
    title TEXT NOT NULL,              -- Título corto o asunto
    message TEXT NOT NULL,            -- Mensaje completo de la notificación
    is_read BOOLEAN DEFAULT FALSE,   -- Si el usuario ya leyó la notificación
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índice para consultas rápidas por usuario y estado leído
CREATE INDEX IF NOT EXISTS idx_notifications_user_read ON notifications(user_id, is_read);
