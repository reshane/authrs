// module declarations
pub mod auth;
pub mod config;
pub mod error;
pub mod types;
mod store;

// internal imports
pub use crate::store::MemStore;
use crate::store::Store;
use crate::auth::google_auth::GoogleAuthClient;
use crate::error::AuthrError;
use crate::types::{User, DataType};

// imports
use axum::{
    extract::{Path, State}, handler::HandlerWithoutStateExt, http::StatusCode, response::IntoResponse, routing::{get, post}, Router
};
use oauth2::PkceCodeVerifier;
use std::{collections::HashMap, sync::Arc, sync::Mutex};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use axum::Json;

// state type
#[derive(Debug)]
pub struct AuthrState {
    sessions: Mutex<HashMap<String, PkceCodeVerifier>>,
    client: GoogleAuthClient,
    store: Mutex<MemStore<User>>,
}

impl AuthrState {
    pub fn new(client: GoogleAuthClient, store: Mutex<MemStore<User>>) -> Self {
        Self {
            sessions: Mutex::new(HashMap::<String, PkceCodeVerifier>::new()),
            client,
            store,
        }
    }
}

// helper functions
async fn handle_not_found() -> impl IntoResponse {
    AuthrError::NotAuthorized.into_response()
}

async fn data_get(
    Path((_data_type, id)): Path<(String, i64)>,
    State(state): State<Arc<AuthrState>>,
) -> impl IntoResponse {
    match state.store.lock() {
        Ok(store) => {
            match store.get(id) {
                Some(data) => Json(data).into_response(),
                None => AuthrError::NotFound.into_response(),
            }
        },
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "could not get data from store").into_response()
        },
    }
}

async fn data_create(
    Path(data_type): Path<DataType>,
    State(state): State<Arc<AuthrState>>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    match data_type {
        DataType::User => {
            match state.store.lock() {
                Ok(mut store) => {
                    match store.create(payload) {
                        Ok(data) => Json(data.clone()).into_response(),
                        Err(_) => AuthrError::NotFound.into_response(),
                    }
                },
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "could not get data from store").into_response()
                }
            }
        },
    }
}

fn data_routes(state: Arc<AuthrState>) -> Router {
    Router::new()
        .route("/{type}/{id}", get(data_get))
        .route("/{type}", post(data_create))
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
