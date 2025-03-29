use std::{collections::HashMap, sync::Arc, sync::Mutex};

use axum::{extract::{Query, State}, handler::HandlerWithoutStateExt, http::StatusCode, response::{self, IntoResponse}, routing::get, Router};
use tokio::net::TcpListener;
use std::env;
use oauth2::{basic::BasicClient, reqwest, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl};
use oauth2::{
    Client,
    basic::{
        BasicRevocationErrorResponse,
        BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenIntrospectionResponse,
    },
    StandardRevocableToken,
};
use tower_http::services::ServeDir;

async fn login(State(state): State<Arc<AuthrState>>) -> impl IntoResponse {
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = state.client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // state.sessions.lock().unwrap().insert(csrf_token.into_secret(), pkce_verifier);
    match state.sessions.lock() {
        Ok(mut sessions) => {
            sessions.insert(csrf_token.into_secret(), pkce_verifier);
        },
        Err(_e) => {
            return response::Redirect::permanent("/");
        }
    };

    response::Redirect::temporary(auth_url.as_str())
}

async fn callback(Query(params): Query<HashMap<String, String>>, State(state): State<Arc<AuthrState>>) -> impl IntoResponse {
    let csrf_token_header = params.get("state");
    let token = match csrf_token_header {
        Some(token) => token,
        None => {
            return (StatusCode::FORBIDDEN, "Not Authorized").into_response();
        },
    };

    let code_header = params.get("code");
    let code = match code_header {
        Some(code) => code.to_string(),
        None => {
            return (StatusCode::FORBIDDEN, "Not Authorized").into_response();
        },
    };

    let pkce_verifier = match state.sessions.lock() {
        Ok(mut sessions) => {
            sessions.remove(token.as_str())
        },
        Err(e) => {
            return (StatusCode::FORBIDDEN, format!("{:?}", e)).into_response();
        }
    };

    let pkce_verifier = match pkce_verifier {
        Some(verifier) => verifier,
        None => {
            return (StatusCode::FORBIDDEN, "Not Authorized").into_response();
        },
    };

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_token`.

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Now you can trade it for an access token.
    let token_result = state.client
        .exchange_code(AuthorizationCode::new(code))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await
        .unwrap();
    
    let oauth_google_url_api = "https://www.googleapis.com/oauth2/v2/userinfo";

    let user_data = http_client.get(oauth_google_url_api)
        .query(&[("access_token", token_result.access_token().secret())])
        .send()
        .await;
    let user_data = match user_data {
        Ok(user_data) => {
            user_data.text().await
        },
        Err(e) => {
            return (StatusCode::FORBIDDEN, e.to_string()).into_response();
        },
    };
    match user_data {
        Ok(user_data) => {
            (StatusCode::OK, [("Content-Type", "application/json")], user_data).into_response()
        },
        Err(e) => {
            return (StatusCode::FORBIDDEN, e.to_string()).into_response();
        }
    }
}

#[derive(Debug)]
struct AuthrState {
    sessions: Mutex<HashMap<String, PkceCodeVerifier>>,
    client: SetClient,
}

// there has to be a way to get rid of this
type SetClient<
    HasAuthUrl = EndpointSet,
    HasDeviceAuthUrl = EndpointNotSet,
    HasIntrospectionUrl = EndpointNotSet,
    HasRevocationUrl = EndpointNotSet,
    HasTokenUrl = EndpointSet,
> = Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    HasAuthUrl,
    HasDeviceAuthUrl,
    HasIntrospectionUrl,
    HasRevocationUrl,
    HasTokenUrl,
>;

#[tokio::main]
async fn main() {
    let sessions = Mutex::new(HashMap::<String, PkceCodeVerifier>::new());
    let client_id = env::var("GOOGLE_OAUTH_CLIENT_ID").expect("client id");
    let client_secret = env::var("GOOGLE_OAUTH_CLIENT_SECRET").expect("client secret");
    let auth_uri = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).expect("auth_uri");
    let token_uri = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).expect("token_uri");
    let redirect_uri = RedirectUrl::new("http://localhost:8080/auth/google/callback".to_string())
        .expect("redirect_uri");

    let client = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(auth_uri)
        .set_token_uri(token_uri)
        .set_redirect_uri(redirect_uri);
    let state = AuthrState {
        sessions,
        client,
    };

    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not Found")
    }

    let state = Arc::new(state);
    let app = Router::new()
        .route("/auth/google/login", get(login))
        .route("/auth/google/callback", get(callback))
        .with_state(state)
        .fallback_service(ServeDir::new("static").not_found_service(handle_404.into_service()));

    let address = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.unwrap();
}
