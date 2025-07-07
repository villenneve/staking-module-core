# Etapa 1: Build com imagem oficial do Rust
FROM rust:latest

# Crie diretório para o projeto
WORKDIR /usr/src/app

# Copie arquivos para o container
COPY . .

# Instale dependências sem compilar o projeto inteiro (melhora cache)
RUN cargo fetch

# Compile o binário em modo release
RUN cargo build --release

# Etapa 2: Container final leve com Debian slim
FROM debian:bookworm-slim

# Instale apenas as dependências mínimas para execução
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Crie usuário não-root por segurança
RUN useradd -m appuser

# Copie o binário do estágio anterior
COPY --from=builder /usr/src/app/target/release/staking-module-core /usr/local/bin/app

# Use o usuário não-root
USER appuser

# Porta usada pelo Actix Web (ajuste se necessário)
EXPOSE 8080

# Comando de inicialização
CMD ["app"]
