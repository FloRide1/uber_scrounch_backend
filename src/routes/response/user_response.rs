use axum::{response::IntoResponse, Json};
use serde_derive::{Deserialize, Serialize};

use crate::models::user_model::UserModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,

    pub email: String,

    pub ban: Option<bool>,
}

impl Into<UserResponse> for UserModel {
    fn into(self) -> UserResponse {
        UserResponse {
            id: self.id,
            email: self.email,
            ban: if self.banned { Some(true) } else { None },
        }
    }
}

impl Into<UserResponse> for &UserModel {
    fn into(self) -> UserResponse {
        UserResponse {
            id: self.id,
            email: self.email.clone(),
            ban: if self.banned { Some(true) } else { None },
        }
    }
}

impl IntoResponse for UserResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
