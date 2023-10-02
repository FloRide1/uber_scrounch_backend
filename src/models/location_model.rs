use std::time::SystemTime;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{schema::locations, DbConnection};

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Queryable, Selectable, Identifiable, AsChangeset,
)]
#[diesel(table_name = locations)]
pub struct LocationModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = name)]
    pub name: String,

    #[diesel(column_name = updated_at)]
    pub updated_at: SystemTime,
}

impl LocationModel {
    pub fn get(conn: &mut DbConnection, id: i32) -> Result<Self, diesel::result::Error> {
        locations::table
            .filter(locations::id.eq(id as i32))
            .first::<Self>(conn)
    }

    pub fn get_list(
        conn: &mut DbConnection,
        ids: Vec<i32>,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        locations::table
            .filter(locations::id.eq_any(ids))
            .get_results(conn)
    }

    pub fn exist(conn: &mut DbConnection, id: i32) -> bool {
        Self::get(conn, id).is_ok()
    }

    pub fn list(conn: &mut DbConnection) -> Result<Vec<i32>, diesel::result::Error> {
        locations::table
            .select(locations::id)
            .get_results::<i32>(conn)
    }
}
