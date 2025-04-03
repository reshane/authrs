use crate::AuthrState;
use axum::Router;
use std::sync::Arc;

pub mod google_auth;

pub fn routes(state: Arc<AuthrState>) -> Router {
    Router::new().nest_service("/google/", google_auth::routes(state))
}

