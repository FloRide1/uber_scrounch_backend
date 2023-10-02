use async_session::{MemoryStore, SessionStore};
use axum::{
    async_trait,
    extract::{rejection::TypedHeaderRejectionReason, FromRef, FromRequestParts, TypedHeader},
    headers::Cookie,
    RequestPartsExt,
};
use hyper::http::request::Parts;
use serde_derive::{Serialize, Deserialize};

use crate::models::user_model::UserModel;

use super::{COOKIE_NAME, OauthRedirect, admin::Admin};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,

    pub email: String,
}

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            email: value.email
        }
    }
}

impl From<Admin> for User {
    fn from(value: Admin) -> Self {
        Self { id: value.id, email: value.email }
    }

}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    MemoryStore: axum::extract::FromRef<S> ,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = OauthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                hyper::header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => OauthRedirect,
                    _ => panic!("Unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("Unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(OauthRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(OauthRedirect)?;

        if let Some(admin) = session.get::<Admin>("admin") {
            return Ok(User::from(admin));
        }

        let user = session.get::<Self>("user").ok_or(OauthRedirect)?;
        Ok(user)
    }
}
