-- Your SQL goes here
-- migrations/sql/up.sql
CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),                 -- ID interno (clave técnica)
    
    id_number VARCHAR(15) NOT NULL UNIQUE,      -- Identificación del usuario (única)
    name VARCHAR(30) NOT NULL,
    lastname VARCHAR(30) NOT NULL,
    
    email VARCHAR(100) NOT NULL UNIQUE,         -- Email también único
    password VARCHAR NOT NULL,                  -- Idealmente encriptado (bcrypt, argon2, etc.)
    
    rol VARCHAR(20) NOT NULL,                   -- Puede ser: "admin", "evaluador", etc.
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


-- Si frecuentemente filtras o haces JOIN por rol (por ejemplo: obtener todos los usuarios "analyst")
CREATE INDEX idx_users_rol ON users(rol);

-- Si haces muchas búsquedas por nombre o apellido (aunque esto a veces se hace con ILIKE y necesita otro tipo de índice)
CREATE INDEX idx_users_name ON users(name);
CREATE INDEX idx_users_lastname ON users(lastname);
