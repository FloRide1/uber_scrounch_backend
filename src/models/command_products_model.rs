use std::time::SystemTime;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{schema::command_products, DbConnection};

use super::{command_model::CommandModel, product_model::ProductModel};

#[derive(
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Associations,
    Queryable,
    Selectable,
    Identifiable,
    AsChangeset,
)]
#[diesel(belongs_to(CommandModel, foreign_key = command_id))]
#[diesel(belongs_to(ProductModel, foreign_key = product_id))]
#[diesel(primary_key(id))]
#[diesel(table_name = command_products)]
pub struct CommandProductModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = command_id)]
    pub command_id: i32,

    #[diesel(column_name = product_id)]
    pub product_id: i32,

    #[diesel(column_name = amount)]
    pub amount: i32,

    #[diesel(column_name = created_at)]
    pub created_at: SystemTime,

    #[diesel(column_name = updated_at)]
    pub updated_at: SystemTime,
}

impl CommandProductModel {
    pub fn new(
        conn: &mut DbConnection,
        new_command_product: NewCommandProductModel,
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(command_products::table)
            .values(new_command_product)
            .get_result::<Self>(conn)
    }

    pub fn new_list(
        conn: &mut DbConnection,
        new_command_product: Vec<NewCommandProductModel>,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        diesel::insert_into(command_products::table)
            .values(new_command_product)
            .get_results::<Self>(conn)
    }

    pub fn get(conn: &mut DbConnection, id: i32) -> Result<Self, diesel::result::Error> {
        command_products::table
            .filter(command_products::id.eq(id))
            .first::<Self>(conn)
    }

    pub fn exist(conn: &mut DbConnection, id: i32) -> bool {
        Self::get(conn, id).is_ok()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[diesel(table_name = command_products)]
pub struct NewCommandProductModel {
    #[diesel(column_name = command_id)]
    pub command_id: i32,

    #[diesel(column_name = product_id)]
    pub product_id: i32,

    #[diesel(column_name = amount)]
    pub amount: i32,
}
