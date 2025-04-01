use tokio::net::TcpListener;
use std::sync::Mutex;

use authrs::{auth::google_auth::GoogleAuthClient, run, AuthrState, MemStore};

#[tokio::main]
async fn main() {
    let client = GoogleAuthClient::from_env();
    let mem_store = Mutex::new(MemStore::new());
    let state = AuthrState::new(client, mem_store);
    /*
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::Layer::default().compact())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.db.get_connection_string().as_str())
        .await
        .expect("couldn't connect to the database");
    */

    let address = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind address");

    run(listener, state).await
}
