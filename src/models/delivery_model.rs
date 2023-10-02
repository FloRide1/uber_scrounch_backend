use std::time::SystemTime;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{schema::deliveries, DbConnection};

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Queryable, Selectable, Identifiable, AsChangeset,
)]
#[diesel(table_name = deliveries)]
pub struct DeliveryModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = time)]
    pub time: SystemTime,

    #[diesel(column_name = updated_at)]
    pub updated_at: SystemTime,
}

impl DeliveryModel {
    pub fn new(
        conn: &mut DbConnection,
        new_delivery: NewDeliveryModel,
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(deliveries::table)
            .values(new_delivery)
            .get_result::<Self>(conn)
    }

    pub fn get(conn: &mut DbConnection, id: i32) -> Result<Self, diesel::result::Error> {
        deliveries::table
            .filter(deliveries::id.eq(id))
            .first::<Self>(conn)
    }

    pub fn get_list(
        conn: &mut DbConnection,
        ids: Vec<i32>,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        deliveries::table
            .filter(deliveries::id.eq_any(ids))
            .get_results(conn)
    }

    pub fn exist(conn: &mut DbConnection, id: i32) -> bool {
        Self::get(conn, id).is_ok()
    }

    pub fn list(conn: &mut DbConnection) -> Result<Vec<i32>, diesel::result::Error> {
        deliveries::table
            .select(deliveries::id)
            .get_results::<i32>(conn)
    }

    pub fn get_next(conn: &mut DbConnection) -> Result<Self, diesel::result::Error> {
        deliveries::table
            .filter(deliveries::time.ge(SystemTime::now()))
            .order_by(deliveries::time)
            .first::<Self>(conn)
    }

    pub fn get_futures(conn: &mut DbConnection) -> Result<Vec<Self>, diesel::result::Error> {
        deliveries::table
            .filter(deliveries::time.ge(SystemTime::now()))
            .order_by(deliveries::time)
            .get_results::<Self>(conn)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[diesel(table_name = deliveries)]
pub struct NewDeliveryModel {
    #[diesel(column_name = time)]
    pub time: SystemTime,
}
