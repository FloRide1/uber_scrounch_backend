use axum::{response::IntoResponse, Json};
use serde_derive::{Deserialize, Serialize};

use crate::{
    models::{
        command_model::CommandModel, location_model::LocationModel, product_model::ProductModel,
        user_model::UserModel,
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

    pub items: Vec<CommandItemResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandItemResponse {
    pub id: i32,

    pub product_name: String,

    pub amount: i32,
}

impl CommandModel {
    pub fn into_response(
        &self,
        conn: &mut DbConnection,
    ) -> Result<CommandResponse, diesel::result::Error> {
        let products = self.get_products(conn)?;
        let item_ids = products.iter().map(|x| x.product_id).collect();
        let products_name = ProductModel::get_list(conn, item_ids)?;
        let location = LocationModel::get(conn, self.location_id)?;
        let user = UserModel::get(conn, self.user_id)?;

        let items = products
            .iter()
            .zip(products_name.iter())
            .map(|(command_product, product)| CommandItemResponse {
                id: command_product.product_id,
                product_name: product.name.clone(),
                amount: command_product.amount,
            })
            .collect();

        Ok(CommandResponse {
            id: self.id,
            location_name: location.name,
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
        write!(
            f,
            "Command \"{}\" for \"{}\" at \"{}\": ",
            self.id, self.user_email, self.location_name,
        )?;
        for i in &self.items {
            write!(f, "{} ", i)?;
        }
        write!(f, "")
    }
}

impl std::fmt::Display for CommandItemResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" × {}", self.product_name, self.amount)
    }
}
