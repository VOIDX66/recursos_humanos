-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS pgcrypto;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM users WHERE rol = 'admin') THEN
        -- Insertar el primer usuario administrador
        INSERT INTO users (id_number, name, lastname, email, password, rol)
        VALUES 
            ('admin001', 'Admin', 'User', 'admin@company.com', 
            crypt('adminpassword', gen_salt('bf', 8)), 'admin');
    END IF;
END $$;