# Etapa final mínima
FROM debian:buster-slim

# Copia o binário compilado com MUSL
COPY target/x86_64-unknown-linux-musl/release/staking-module-core /usr/local/bin/staking-core

# Define a porta padrão (caso seu app use)
EXPOSE 8080

# Comando de execução
CMD ["staking-core"]
