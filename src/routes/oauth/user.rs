use async_session::{MemoryStore, SessionStore};
use axum::{
    async_trait,
    extract::{rejection::TypedHeaderRejectionReason, FromRef, FromRequestParts, TypedHeader},
    headers::Cookie,
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use hyper::{http::request::Parts, StatusCode};
use serde_derive::{Deserialize, Serialize};

use crate::{models::user_model::UserModel, state::PoolType};

use super::{admin::Admin, OauthRedirect, COOKIE_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,

    pub email: String,

    pub banned: bool,
}

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            email: value.email,
            banned: value.banned,
        }
    }
}

impl From<Admin> for User {
    fn from(value: Admin) -> Self {
        Self {
            id: value.id,
            email: value.email,
            banned: false,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    PoolType: axum::extract::FromRef<S>,
    MemoryStore: axum::extract::FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookies =
            parts
                .extract::<TypedHeader<Cookie>>()
                .await
                .map_err(|e| match *e.name() {
                    hyper::header::COOKIE => match e.reason() {
                        TypedHeaderRejectionReason::Missing => OauthRedirect.into_response(),
                        _ => panic!("Unexpected error getting Cookie header(s): {e}"),
                    },
                    _ => panic!("Unexpected error getting cookies: {e}"),
                })?;
        let session_cookie = cookies
            .get(COOKIE_NAME)
            .ok_or(OauthRedirect.into_response())?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(OauthRedirect.into_response())?;

        if let Some(admin) = session.get::<Admin>("admin") {
            return Ok(User::from(admin));
        }

        let user = session
            .get::<Self>("user")
            .ok_or(OauthRedirect.into_response())?;

        let pool = PoolType::from_ref(state);
        pool.get()
            .await
            .unwrap()
            .interact(move |conn| {
                let user_model = UserModel::get(conn, user.id).map_err(|err| {
                    error!("{}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Something unexpected happened",
                    )
                        .into_response()
                })?;
                if user_model.banned {
                    return Err((StatusCode::FORBIDDEN, "You're banned").into_response());
                }
                Ok(user)
            })
            .await
            .unwrap()
    }
}
