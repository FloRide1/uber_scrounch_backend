pub mod get;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::product_model::NewProductModel;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiRoot {
    pub data: Vec<ApiItem>,
    pub limit: String,
    pub start: String,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiItem {
    pub category: Category,
    pub code: String,
    pub id: String,
    pub image_url: String,
    pub name: String,
    pub net_price: String,
    pub price: String,
    pub slug: String,
    pub tax_method: String,
    // pub tax_rate: TaxRate,
    #[serde(rename = "type")]
    pub type_field: String,
    // pub unit: Unit,
    pub unit_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub code: String,
    pub name: String,
    pub image: Value,
    pub parent_id: String,
    pub slug: String,
    pub description: String,
}

impl Into<NewProductModel> for ApiItem {
    fn into(self) -> NewProductModel {
        NewProductModel {
            sma_id: self.id,
            name: self.name,
            stock: 0,
            price: self.price.parse().unwrap(),
            image_url: self.image_url,
        }
    }
}
