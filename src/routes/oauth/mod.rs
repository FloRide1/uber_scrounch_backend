pub mod admin;
pub mod login;
pub mod microsoft;
pub mod user;

use anyhow::Context;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use super::AppError;

pub static COOKIE_NAME: &str = "SESSION";

pub fn oauth_client() -> Result<BasicClient, AppError> {
    let client_id = std::env::var("CLIENT_ID").context("Missing CLIENT_ID")?;
    let client_secret = std::env::var("CLIENT_SECRET").context("Missing CLIENT_SECRET")?;
    let redirect_url = std::env::var("REDIRECT_URL").context("Missing REDIRECT_URL")?;

    let auth_url = std::env::var("AUTH_URL").context("Missing AUTH_URL")?;
    let token_url = std::env::var("TOKEN_URL").context("Missing TOKEN_URL")?;

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).context("Failed to create new authorization server URL")?,
        Some(TokenUrl::new(token_url).context("Failed to create new token endpoint URL")?),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url).context("Failed to create new redirection URL")?,
    ))
}

pub struct OauthRedirect;
impl axum::response::IntoResponse for OauthRedirect {
    fn into_response(self) -> axum::response::Response {
        axum::response::Redirect::temporary("/auth/microsoft").into_response()
    }
}
