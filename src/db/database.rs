use mongodb::{options::ClientOptions, Client};

/// Conecta ao MongoDB usando URI do ambiente
pub async fn connect_db() -> Client {
    let db_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not set");
    let options = ClientOptions::parse(db_uri).await.expect("Erro ao conectar no MongoDB");
    Client::with_options(options).expect("Erro ao criar client")
}
