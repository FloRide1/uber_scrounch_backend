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

impl Into<ProductResponse> for ProductModel {
    fn into(self) -> ProductResponse {
        ProductResponse {
            id: self.id,
            name: self.name,
            description: self.description,
            image_url: "https://picsum.photos/250/250?random".to_string(),
            stock: self.stock,
            price: self.price,
        }
    }
}

impl Into<ProductResponse> for &ProductModel {
    fn into(self) -> ProductResponse {
        ProductResponse {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            image_url: "https://picsum.photos/250/250?random".to_string(),
            stock: self.stock,
            price: self.price,
        }
    }
}

impl IntoResponse for ProductResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
