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
    extract::{Path, State}, handler::HandlerWithoutStateExt, response::IntoResponse, routing::{get, post}, Router
};
use std::{collections::HashMap, sync::Arc, sync::Mutex};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use axum::Json;
use axum::routing::delete;
use tracing::info;

// state type
#[derive(Debug)]
pub struct AuthrState {
    sessions: Mutex<HashMap<String, String>>,
    google_client: GoogleAuthClient,
    store: MemStore,
}

impl AuthrState {
    pub fn new(google_client: GoogleAuthClient, store: MemStore) -> Self {
        Self {
            sessions: Mutex::new(HashMap::<String, String>::new()),
            google_client,
            store,
        }
    }
}

// helper functions
async fn handle_not_found() -> impl IntoResponse {
    AuthrError::NotFound.into_response()
}

async fn data_get(
    Path((data_type, id)): Path<(DataType, i64)>,
    State(state): State<Arc<AuthrState>>,
) -> impl IntoResponse {
    match state.store.get(id, data_type).await {
        Some(data) => {
            let downcasted = data.as_any().downcast_ref::<User>();
            if let Some(ins_data) = downcasted {
                return Json(ins_data.clone()).into_response()
            }
            AuthrError::NotFound.into_response()
        },
        None => AuthrError::NotFound.into_response(),
    }
}

async fn data_delete(
    Path((data_type, id)): Path<(DataType, i64)>,
    State(state): State<Arc<AuthrState>>,
) -> impl IntoResponse {
    match state.store.delete(id, data_type).await {
        Ok(data) => {
            let downcasted = data.as_any().downcast_ref::<User>();
            if let Some(ins_data) = downcasted {
                return Json(ins_data.clone()).into_response()
            }
            AuthrError::NotFound.into_response()
        },
        Err(_) => AuthrError::NotFound.into_response(),
    }
}

async fn data_create(
    Path(data_type): Path<DataType>,
    State(state): State<Arc<AuthrState>>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    match data_type {
        DataType::User => {
            match state.store.create(&payload).await {
                Ok(data) => {
                    let downcasted = data.as_any().downcast_ref::<User>();
                    if let Some(ins_data) = downcasted {
                        return Json(ins_data.clone()).into_response()
                    }
                    AuthrError::NotFound.into_response()
                },
                Err(_) => AuthrError::NotFound.into_response(),
            }
        },
    }
}

fn data_routes(state: Arc<AuthrState>) -> Router {
    Router::new()
        .route("/{type}/{id}", get(data_get))
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
