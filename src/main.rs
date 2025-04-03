use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;

use authrs::{AuthrState, PsqlStore, auth::google_auth::GoogleAuthClient, run};
use tracing::info;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypass@localhost/mydb")
        .await
        .expect("couldn't connect to the database");
    let client = GoogleAuthClient::from_env();
    // let mem_store = MemStore::new();
    let psql_store = PsqlStore::new(pool);
    let state = AuthrState::new(client, psql_store);

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

    let address = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind address");

    run(listener, state).await
}
