use async_session::{MemoryStore, SessionStore};
use axum::{
    async_trait,
    extract::{rejection::TypedHeaderRejectionReason, FromRef, FromRequestParts, TypedHeader},
    headers::Cookie,
    RequestPartsExt, response::{Response, IntoResponse},
};
use hyper::http::request::Parts;
use serde_derive::{Serialize, Deserialize};

use crate::models::user_model::UserModel;

use super::{COOKIE_NAME, OauthRedirect, user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    pub id: i32,

    pub email: String,
}

impl From<UserModel> for Admin {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            email: value.email
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Admin
where
    MemoryStore: axum::extract::FromRef<S> ,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                hyper::header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => OauthRedirect.into_response(),
                    _ => panic!("Unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("Unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(OauthRedirect.into_response())?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(OauthRedirect.into_response())?;

        match session.get::<Self>("admin") {
            Some(admin) => Ok(admin),
            None => {
                if session.get::<User>("user").is_some() {
                    Err((hyper::StatusCode::FORBIDDEN, "Only Admin can access this ressource").into_response())
                }
                else {
                    Err(OauthRedirect.into_response())
                }
            }
        }
    }
}
