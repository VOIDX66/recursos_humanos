-- Your SQL goes here
-- migrations/sql/up.sql
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,                 -- ID interno (clave técnica)
    
    id_number VARCHAR(15) NOT NULL UNIQUE,      -- Identificación del usuario (única)
    name VARCHAR(30) NOT NULL,
    lastname VARCHAR(30) NOT NULL,
    
    email VARCHAR(100) NOT NULL UNIQUE,         -- Email también único
    password VARCHAR NOT NULL,                  -- Idealmente encriptado (bcrypt, argon2, etc.)
    
    rol VARCHAR(20) NOT NULL,                   -- Puede ser: "admin", "evaluador", etc.
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


