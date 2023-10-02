use axum::{response::IntoResponse, Json};
use serde_derive::{Deserialize, Serialize};

use crate::models::location_model::LocationModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
    pub id: i32,

    pub name: String,
}

impl Into<LocationResponse> for LocationModel {
    fn into(self) -> LocationResponse {
        LocationResponse {
            id: self.id,
            name: self.name,
        }
    }
}

impl Into<LocationResponse> for &LocationModel {
    fn into(self) -> LocationResponse {
        LocationResponse {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

impl IntoResponse for LocationResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
