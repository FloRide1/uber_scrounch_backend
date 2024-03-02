use axum::{response::IntoResponse, Json};
use serde_derive::{Deserialize, Serialize};

use crate::models::delivery_model::DeliveryModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryResponse {
    pub id: i32,

    pub time: u128,
}

impl From<DeliveryModel> for DeliveryResponse {
    fn from(val: DeliveryModel) -> Self {
        Self {
            id: val.id,
            time: val
                .time
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }
}

impl From<&DeliveryModel> for DeliveryResponse {
    fn from(val: &DeliveryModel) -> Self {
        Self {
            id: val.id,
            time: val
                .time
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }
}

impl IntoResponse for DeliveryResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
