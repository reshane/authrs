// module declarations
pub mod auth;
pub mod config;
pub mod error;
mod store;
pub mod types;

// internal imports
use crate::auth::google_auth::GoogleAuthClient;
use crate::error::AuthrError;
pub use crate::store::PsqlStore;
use crate::store::Storeable;
use crate::types::{DataType, User};

// imports
use axum::{
    Json, Router,
    extract::{Path, State},
    handler::HandlerWithoutStateExt,
    response::IntoResponse,
    routing::{delete, get, post},
};
use std::{collections::HashMap, sync::Arc, sync::Mutex};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

// state type
#[derive(Debug)]
pub struct AuthrState {
    sessions: Mutex<HashMap<String, String>>,
    google_client: GoogleAuthClient,
    store: Arc<PsqlStore>,
}

impl AuthrState {
    pub fn new(google_client: GoogleAuthClient, store: PsqlStore) -> Self {
        Self {
            sessions: Mutex::new(HashMap::<String, String>::new()),
            google_client,
            store: Arc::new(store),
        }
    }
}

async fn data_get_queries(
    Path(data_type): Path<DataType>,
    State(state): State<Arc<AuthrState>>,
) -> impl IntoResponse {
    match data_type {
        DataType::User => {
            let data = User::get_queries(&HashMap::new(), state.store.clone().as_ref()).await;
            Json(data.clone()).into_response()
        }
    }
}

async fn data_get(
    Path((data_type, id)): Path<(DataType, i64)>,
    State(state): State<Arc<AuthrState>>,
) -> impl IntoResponse {
    match data_type {
        DataType::User => {
            let data = User::get(id, state.store.clone().as_ref()).await;
            match data {
                Some(data) => Json(data.clone()).into_response(),
                None => AuthrError::NotFound.into_response(),
            }
        }
    }
}

async fn data_delete(
    Path((data_type, id)): Path<(DataType, i64)>,
    State(state): State<Arc<AuthrState>>,
) -> impl IntoResponse {
    match data_type {
        DataType::User => {
            let data = User::delete(id, state.store.clone().as_ref()).await;
            match data {
                Ok(data) => Json(data.clone()).into_response(),
                Err(_) => AuthrError::NotFound.into_response(),
            }
        }
    }
}

async fn data_create(
    Path(data_type): Path<DataType>,
    State(state): State<Arc<AuthrState>>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    match data_type {
        DataType::User => {
            let data = payload.create(state.store.clone().as_ref()).await;
            match data {
                Ok(data) => Json(data.clone()).into_response(),
                Err(_) => AuthrError::NotFound.into_response(),
            }
        }
    }
}

// helper functions
async fn handle_not_found() -> impl IntoResponse {
    AuthrError::NotFound.into_response()
}

fn data_routes(state: Arc<AuthrState>) -> Router {
    Router::new()
        .route("/{type}/{id}", get(data_get))
        .route("/{type}", get(data_get_queries))
        .route("/{type}/{id}", delete(data_delete))
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

    info!("Listening on {:?}", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
