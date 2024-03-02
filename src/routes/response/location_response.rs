use axum::{response::IntoResponse, Json};
use serde_derive::{Deserialize, Serialize};

use crate::models::location_model::LocationModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
    pub id: i32,

    pub name: String,
}

impl From<LocationModel> for LocationResponse {
    fn from(val: LocationModel) -> Self {
        Self {
            id: val.id,
            name: val.name,
        }
    }
}

impl From<&LocationModel> for LocationResponse {
    fn from(val: &LocationModel) -> Self {
        Self {
            id: val.id,
            name: val.name.clone(),
        }
    }
}

impl IntoResponse for LocationResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
