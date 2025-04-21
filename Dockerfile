# Etapa 1: Compilación
FROM rust:slim as builder

# Crear un directorio para la aplicación
WORKDIR /usr/src/app

# Instalar dependencias del sistema necesarias para compilar Diesel
RUN apt-get update && apt-get install -y libpq-dev pkg-config

# Instalar Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Copiar el proyecto completo
COPY . .

# Compilar la aplicación en modo release
RUN cargo build --release

# Etapa 2: Producción
FROM debian:bookworm-slim

# Instalar dependencias necesarias en tiempo de ejecución
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    postgresql-client && \
    rm -rf /var/lib/apt/lists/*

# Crear directorio para la app
WORKDIR /app

# Copiar binario de la app y Diesel CLI desde el builder
COPY --from=builder /usr/src/app/target/release/recursos_humanos_back /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/

# Copiar migraciones y config de Diesel
COPY --from=builder /usr/src/app/migrations /app/migrations
COPY --from=builder /usr/src/app/diesel.toml /app/

# Exponer puerto
EXPOSE 4500

# Usar bash como entrypoint y ejecutar comandos Diesel y app
ENTRYPOINT bash -c '\
    echo "Esperando a que PostgreSQL esté listo..." && \
    until psql "$DATABASE_URL" -c "\q" >/dev/null 2>&1; do \
        echo "Esperando..." && sleep 1; \
    done && \
    echo "PostgreSQL listo, ejecutando migraciones..." && \
    diesel migration run --migration-dir ./migrations && \
    echo "Iniciando aplicación..." && \
    exec recursos_humanos_back'
