# Etapa de compilación
FROM rust:slim as builder

# Instala dependencias del sistema
RUN apt-get update && apt-get install -y libpq-dev pkg-config build-essential

# Crea directorio para el proyecto
WORKDIR /app

# Copia archivos de configuración y dependencias primero (mejor uso de cache)
COPY Cargo.toml Cargo.lock ./
COPY diesel.toml ./

# Crea un dummy para compilar dependencias más rápido
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -r src

# Copia el resto del código
COPY . .

# Instala diesel_cli para usar en esta etapa
RUN cargo install diesel_cli --no-default-features --features postgres
# Compila la app en modo release
RUN cargo build --release

# Etapa final: imagen más liviana
FROM debian:bookworm-slim

# Instala solo lo necesario para correr la app y conectarse a Postgres
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Crea usuario no root por seguridad (opcional pero recomendado)
RUN useradd -m appuser

WORKDIR /app

# Copia ejecutable y migraciones desde el builder
COPY --from=builder /app/target/release/recursos_humanos_back .
COPY --from=builder /app/migrations ./migrations

# Copia el binario diesel para ejecutar migraciones
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Usa usuario no root
USER appuser

# Expon el puerto (solo informativo)
EXPOSE 4500

# Comando por defecto (puede sobreescribirse en docker-compose)
CMD ["./recursos_humanos_back"]
