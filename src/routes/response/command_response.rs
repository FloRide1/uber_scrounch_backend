use axum::{response::IntoResponse, Json};
use serde_derive::{Deserialize, Serialize};

use crate::{
    models::{
        command_model::CommandModel, delivery_model::DeliveryModel, location_model::LocationModel,
        product_model::ProductModel, user_model::UserModel,
    },
    DbConnection,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub id: i32,

    #[serde(skip_serializing)]
    pub location_name: String,

    #[serde(skip_serializing)]
    pub user_id: i32,

    #[serde(skip_serializing)]
    pub user_email: String,

    #[serde(skip_serializing)]
    pub total_price: f64,

    pub confirmed: bool,

    pub delivered: bool,

    pub canceled: bool,

    pub delivery: Option<u128>,

    pub items: Vec<CommandItemResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandItemResponse {
    pub id: i32,

    pub product_name: String,

    pub amount: i32,

    pub price: f64,

    pub image_url: String,
}

impl CommandModel {
    pub fn into_response(
        &self,
        conn: &mut DbConnection,
    ) -> Result<CommandResponse, diesel::result::Error> {
        let products = self.get_products(conn)?;
        let item_ids = products.iter().map(|x| x.product_id);
        let products_name = ProductModel::get_list(conn, item_ids)?;
        let location = LocationModel::get(conn, self.location_id)?;
        let user = UserModel::get(conn, self.user_id)?;

        let mut delivery_time = None;
        if let Some(delivery_id) = self.delivery_id {
            let delivery = DeliveryModel::get(conn, delivery_id)?;
            delivery_time = Some(
                delivery
                    .time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
            );
        }

        let items: Vec<_> = products
            .iter()
            .zip(products_name.iter())
            .map(|(command_product, product)| CommandItemResponse {
                id: command_product.product_id,
                product_name: product.name.clone(),
                amount: command_product.amount,
                image_url: product.image_url.clone(),
                price: product.price,
            })
            .collect();

        Ok(CommandResponse {
            id: self.id,
            location_name: location.name,
            confirmed: self.confirmed,
            delivered: self.delivered,
            canceled: self.canceled,
            delivery: delivery_time,
            total_price: items.iter().fold(0.0, |a, b| a + b.amount as f64 * b.price),
            items,
            user_id: self.user_id,
            user_email: user.email,
        })
    }
}

impl IntoResponse for CommandResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl std::fmt::Display for CommandResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[{}][{}]: \"{}\" : ",
            self.location_name, self.id, self.user_email,
        )?;
        for i in &self.items {
            writeln!(f, "{} ", i)?;
        }
        write!(f, "{}€", self.total_price)
    }
}

impl std::fmt::Display for CommandItemResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} × \"{}\" ", self.amount, self.product_name)
    }
}
