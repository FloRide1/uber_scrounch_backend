use axum::{response::IntoResponse, Json};
use serde_derive::{Deserialize, Serialize};

use crate::models::product_model::ProductModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: i32,

    pub name: String,

    pub description: Option<String>,

    pub image_url: String,

    pub stock: i32,

    pub price: f64,
}

impl From<ProductModel> for ProductResponse {
    fn from(val: ProductModel) -> Self {
        Self {
            id: val.id,
            name: val.name,
            description: val.description,
            image_url: val.image_url,
            stock: val.stock,
            price: val.price,
        }
    }
}

impl From<&ProductModel> for ProductResponse {
    fn from(val: &ProductModel) -> Self {
        Self {
            id: val.id,
            name: val.name.clone(),
            description: val.description.clone(),
            image_url: val.image_url.clone(),
            stock: val.stock,
            price: val.price,
        }
    }
}

impl IntoResponse for ProductResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
