// module declarations
pub mod auth;
pub mod config;
pub mod error;
mod store;
pub mod types;

// internal imports
use crate::auth::google_auth::GoogleAuthClient;
use crate::error::AuthrError;

// imports
use axum::{
    Router,
    extract::{Path, State},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use oauth2::PkceCodeVerifier;
use std::{collections::HashMap, sync::Arc, sync::Mutex};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

// state type
#[derive(Debug)]
pub struct AuthrState {
    sessions: Mutex<HashMap<String, PkceCodeVerifier>>,
    client: GoogleAuthClient,
}

impl AuthrState {
    pub fn new(client: GoogleAuthClient) -> Self {
        Self {
            sessions: Mutex::new(HashMap::<String, PkceCodeVerifier>::new()),
            client,
        }
    }
}

// helper functions
async fn handle_not_found() -> impl IntoResponse {
    AuthrError::NotAuthorized.into_response()
}

async fn data_get(
    Path((data_type, id)): Path<(String, i64)>,
    State(_): State<Arc<AuthrState>>,
) -> impl IntoResponse {
    (StatusCode::OK, format!("{} {}", data_type, id)).into_response()
}

fn data_routes(state: Arc<AuthrState>) -> Router {
    Router::new()
        .route("/{type}/{id}", get(data_get))
        .with_state(state)
}

pub async fn run(listener: TcpListener, state: AuthrState) {
    let state = Arc::new(state);
    let app = Router::new()
        .nest_service("/data/", data_routes(state.clone()))
        .nest_service("/auth/", auth::routes(state))
        .fallback_service(
            ServeDir::new("static").not_found_service(handle_not_found.into_service()),
        );

    axum::serve(listener, app).await.unwrap();
}
