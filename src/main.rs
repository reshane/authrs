use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;
use std::env;

use authrs::{auth::google_auth::GoogleAuthClient, run, AuthrState, MemStore};
use tracing::info;

#[tokio::main]
async fn main() {
    let client = GoogleAuthClient::from_env();
    let mem_store = MemStore::new();
    let state = AuthrState::new(client, mem_store);

    if env::var("RUST_LOG").is_err() {
        panic!("RUST_LOG not set!");
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::Layer::default())
        .init();
    info!("{:?}", env::var("RUST_LOG"));

    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    // tracing::subscriber::set_global_default(subscriber)
        // .expect("Could not set tracing subscriber");
    /*
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
