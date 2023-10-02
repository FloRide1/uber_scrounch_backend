use anyhow::Context;
use async_session::SessionStore;
use axum::{extract::{Query, State}, response::IntoResponse};
use oauth2::TokenResponse;
use serde_derive::{Deserialize, Serialize};

use crate::{routes::{AppError, oauth::COOKIE_NAME}, state::AppState, models::user_model::UserModel};

use super::{user::User, admin::Admin};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MicrosoftUserData {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "mail")]
    mail: String,

    #[serde(rename = "displayName")]
    display_name: Option<String>,
}



pub async fn login() -> axum::response::Response {
    super::OauthRedirect.into_response()
}


pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // Get Token
    let token = state.oauth_client
        .exchange_code(oauth2::AuthorizationCode::new(query.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .context("Failed in sending request request to authorization server")?;

    // Fetch user data from microsoft graph
    let client = reqwest::Client::new();
    let user_data = client
        .get("https://graph.microsoft.com/v1.0/me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("Failed in sending request to target Url")?
        .json::<MicrosoftUserData>()
        .await
        .context("Failed to deserialize response as JSON")?;

    let user = state.pool.get().await.unwrap().interact(move |conn| {
        UserModel::find_or_create_from_email(conn, &user_data.mail)
    }).await.unwrap().context("Cannot create or find User")?;


    // Create a new session filled with user data
    let mut session = async_session::Session::new();
    if user.admin {
        session
            .insert("admin", Admin::from(user))
            .context("Failed in inserting serialized value into session")?;
    } else {
        session
            .insert("user",  User::from(user))
            .context("Failed in inserting serialized value into session")?;
    }


    // Store session and get corresponding cookie
    let cookie = state.store
        .store_session(session)
        .await
        .context("failed to store session")?
        .context("unexpected error retrieving cookie value")?;

    // Build the cookie
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

    // Set cookie
    let mut headers = hyper::HeaderMap::new();
    headers.insert(
        hyper::header::SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    let url = std::env::var("FRONTED_URL").unwrap_or("/".to_string());

    Ok((headers, axum::response::Redirect::to(&url)))
}
