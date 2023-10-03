use std::time::SystemTime;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{schema::users, DbConnection};

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Queryable, Selectable, Identifiable, AsChangeset,
)]
#[diesel(table_name = users)]
pub struct UserModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = email)]
    pub email: String,

    #[diesel(column_name = admin)]
    pub admin: bool,

    #[diesel(column_name = banned)]
    pub banned: bool,

    #[diesel(column_name = updated_at)]
    pub updated_at: SystemTime,
}

impl std::fmt::Display for UserModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "<User {id} \"{email}\">",
            id = self.id,
            email = self.email
        )
    }
}

impl UserModel {
    pub fn new(
        conn: &mut DbConnection,
        new_user: NewUserModel,
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<Self>(conn)
    }

    pub fn get(conn: &mut DbConnection, id: i32) -> Result<Self, diesel::result::Error> {
        users::table
            .filter(users::id.eq(id as i32))
            .first::<Self>(conn)
    }

    pub fn exist(conn: &mut DbConnection, id: i32) -> bool {
        Self::get(conn, id).is_ok()
    }

    pub fn find_from_email(
        conn: &mut DbConnection,
        email: &str,
    ) -> Result<Self, diesel::result::Error> {
        users::table
            .filter(users::email.eq(email))
            .first::<Self>(conn)
    }

    pub fn find_or_create_from_email(
        conn: &mut DbConnection,
        email: &str,
    ) -> Result<Self, diesel::result::Error> {
        match Self::find_from_email(conn, email) {
            Ok(x) => Ok(x),
            Err(err) => match err {
                diesel::result::Error::NotFound => Self::new(
                    conn,
                    NewUserModel {
                        email: email.to_string(),
                    },
                ),
                _ => Err(err),
            },
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserModel {
    #[diesel(column_name = email)]
    pub email: String,
}
