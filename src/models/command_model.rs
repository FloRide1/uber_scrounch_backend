use std::time::SystemTime;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{schema::commands, DbConnection};

use super::{
    command_products_model::CommandProductModel, delivery_model::DeliveryModel,
    location_model::LocationModel, user_model::UserModel,
};

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
#[diesel(belongs_to(UserModel, foreign_key = user_id))]
#[diesel(belongs_to(LocationModel, foreign_key = location_id))]
#[diesel(belongs_to(DeliveryModel, foreign_key = delivery_id))]
#[diesel(table_name = commands)]
pub struct CommandModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = user_id)]
    pub user_id: i32,

    #[diesel(column_name = location_id)]
    pub location_id: i32,

    #[diesel(column_name = delivery_id)]
    pub delivery_id: Option<i32>,

    #[diesel(column_name = confirmed)]
    pub confirmed: bool,

    #[diesel(column_name = delivered)]
    pub delivered: bool,

    #[diesel(column_name = canceled)]
    pub canceled: bool,

    #[diesel(column_name = created_at)]
    pub created_at: SystemTime,

    #[diesel(column_name = updated_at)]
    pub updated_at: SystemTime,
}

impl CommandModel {
    pub fn new(
        conn: &mut DbConnection,
        new_command: NewCommandModel,
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(commands::table)
            .values(new_command)
            .get_result::<Self>(conn)
    }

    pub fn get(conn: &mut DbConnection, id: i32) -> Result<Self, diesel::result::Error> {
        commands::table
            .filter(commands::id.eq(id as i32))
            .first::<Self>(conn)
    }

    pub fn exist(conn: &mut DbConnection, id: i32) -> bool {
        Self::get(conn, id).is_ok()
    }

    pub fn get_products(
        &self,
        conn: &mut DbConnection,
    ) -> Result<Vec<CommandProductModel>, diesel::result::Error> {
        CommandProductModel::belonging_to(&self).get_results(conn)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[diesel(table_name = commands)]
pub struct NewCommandModel {
    #[diesel(column_name = user_id)]
    pub user_id: i32,

    #[diesel(column_name = location_id)]
    pub location_id: i32,
}
