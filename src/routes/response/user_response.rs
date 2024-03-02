use axum::{response::IntoResponse, Json};
use serde_derive::{Deserialize, Serialize};

use crate::models::user_model::UserModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,

    pub email: String,

    pub is_banned: bool,

    pub is_admin: bool,
}

impl From<UserModel> for UserResponse {
    fn from(val: UserModel) -> Self {
        UserResponse {
            id: val.id,
            email: val.email.clone(),
            is_banned: val.banned,
            is_admin: val.admin,
        }
    }
}

impl From<&UserModel> for UserResponse {
    fn from(val: &UserModel) -> Self {
        UserResponse {
            id: val.id,
            email: val.email.clone(),
            is_banned: val.banned,
            is_admin: val.admin,
        }
    }
}

impl IntoResponse for UserResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
