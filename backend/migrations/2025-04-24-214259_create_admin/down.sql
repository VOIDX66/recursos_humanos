-- This file should undo anything in `up.sql`

-- Borrar el primer usuario administrador si existe
DELETE FROM users
WHERE role = 'admin' AND id_number = 'admin001';
