use axum::{response::{IntoResponse, Redirect}, extract::State};
use oauth2::basic::BasicClient;


pub async fn microsoft_auth(State(client): State<BasicClient>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("openid".to_string()))
        .add_scope(oauth2::Scope::new("email".to_string()))
        .add_scope(oauth2::Scope::new("profile".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}


