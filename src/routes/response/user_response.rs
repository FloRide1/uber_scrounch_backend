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

impl Into<UserResponse> for UserModel {
    fn into(self) -> UserResponse {
        UserResponse {
            id: self.id,
            email: self.email,
            is_banned: self.banned,
            is_admin: self.admin,
        }
    }
}

impl Into<UserResponse> for &UserModel {
    fn into(self) -> UserResponse {
        UserResponse {
            id: self.id,
            email: self.email.clone(),
            is_banned: self.banned,
            is_admin: self.admin,
        }
    }
}

impl IntoResponse for UserResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
