use std::time::SystemTime;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{
    schema::products,
    schema::{command_products, commands},
    DbConnection,
};

use super::command_products_model::CommandProductModel;

#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = products)]
pub struct ProductModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = sma_id)]
    pub sma_id: i32,

    #[diesel(column_name = name)]
    pub name: String,

    #[diesel(column_name = description)]
    pub description: Option<String>,

    #[diesel(column_name = price)]
    pub price: f64,

    #[diesel(column_name = stock)]
    pub stock: i32,

    #[diesel(column_name = updated_at)]
    pub updated_at: SystemTime,
}

impl ProductModel {
    pub fn new(
        conn: &mut DbConnection,
        new_product: NewProductModel,
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(products::table)
            .values(new_product)
            .get_result::<Self>(conn)
    }

    pub fn get(conn: &mut DbConnection, id: i32) -> Result<Self, diesel::result::Error> {
        products::table
            .filter(products::id.eq(id))
            .first::<Self>(conn)
    }

    pub fn get_list(
        conn: &mut DbConnection,
        ids: Vec<i32>,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        products::table
            .filter(products::id.eq_any(ids))
            .get_results(conn)
    }

    pub fn exist(conn: &mut DbConnection, id: i32) -> bool {
        Self::get(conn, id).is_ok()
    }

    pub fn list(conn: &mut DbConnection) -> Result<Vec<i32>, diesel::result::Error> {
        products::table
            .select(products::id)
            .get_results::<i32>(conn)
    }

    pub fn get_total_commanded(
        &self,
        conn: &mut DbConnection,
    ) -> Result<Vec<i32>, diesel::result::Error> {
        CommandProductModel::belonging_to(&self)
            .inner_join(commands::table)
            .filter(commands::canceled.eq(false))
            .filter(commands::delivered.eq(false))
            .select(commands::id)
            .get_results::<i32>(conn)
    }

    pub fn get_total_commanded_amount(
        &self,
        conn: &mut DbConnection,
    ) -> Result<i64, diesel::result::Error> {
        CommandProductModel::belonging_to(&self)
            .inner_join(commands::table)
            .filter(commands::canceled.eq(false))
            .filter(commands::delivered.eq(false))
            .select(diesel::dsl::sum(command_products::amount))
            .first(conn)
            .map(|x: Option<i64>| x.unwrap())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[diesel(table_name = products)]
pub struct NewProductModel {
    #[diesel(column_name = sma_id)]
    pub sma_id: i32,

    #[diesel(column_name = stock)]
    pub stock: i32,
}
