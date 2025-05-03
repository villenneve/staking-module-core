# Etapa 1: builder
FROM rust:1.77 as builder

WORKDIR /app

# Copia o código-fonte e dependências
COPY . .

# Compila o binário em modo release
RUN cargo build --release

# Etapa 2: imagem final mínima
FROM debian:buster-slim

# Copia o binário compilado
COPY --from=builder /app/target/release/staking-module-core /usr/local/bin/staking-core

# Porta padrão
EXPOSE 8080

# Comando para iniciar a aplicação
CMD ["staking-core"]
