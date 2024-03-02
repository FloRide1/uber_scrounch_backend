use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::result::Error;

use crate::{models::user_model::UserModel, state::PoolType};

use super::{
    oauth::{admin::Admin, user::User},
    response::user_response::UserResponse,
};

pub async fn me(user: Option<User>, State(pool): State<PoolType>) -> Response {
    match user {
        Some(user) => {
            let id = user.id;
            Into::<UserResponse>::into(
                pool.get()
                    .await
                    .unwrap()
                    .interact(move |conn| UserModel::get(conn, id))
                    .await
                    .unwrap()
                    .unwrap(),
            )
            .into_response()
        }
        None => (
            StatusCode::FORBIDDEN,
            "You're not connected, please connect throught /login",
        )
            .into_response(),
    }
}

pub async fn get_user(
    _admin: Admin,
    Path(id): Path<i32>,
    State(pool): State<PoolType>,
) -> Result<UserResponse, impl IntoResponse> {
    let res = pool
        .get()
        .await
        .unwrap()
        .interact(move |conn| UserModel::get(conn, id))
        .await
        .unwrap();

    match res {
        Ok(res) => Ok(res.into()),
        Err(err) => match err {
            Error::NotFound => Err((
                StatusCode::NOT_FOUND,
                format!("The user with id: \"{id}\" doesn't exist"),
            )),
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something unexpected happened".to_string(),
            )),
        },
    }
}
