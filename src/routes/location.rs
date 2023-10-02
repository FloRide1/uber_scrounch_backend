use axum::{Json, extract::{State, Path}, response::IntoResponse, http::StatusCode};
use diesel::result::Error;

use crate::{state::PoolType, models::location_model::LocationModel};

use super::response::location_response::LocationResponse;


pub async fn get_location_ids(State(pool): State<PoolType>) -> Json<Vec<i32>> {
    let res = pool.get().await.unwrap().interact(move |conn| {
        LocationModel::list(conn).unwrap_or(vec![])
    }).await.unwrap();

    Json(res)
}

pub async fn get_location(Path(id): Path<i32>, State(pool): State<PoolType>) -> Result<LocationResponse, impl IntoResponse> {
    let res = pool.get().await.unwrap().interact(move |conn| {
        LocationModel::get(conn, id)
    }).await.unwrap();

    match res {
        Ok(res) => Ok(res.into()),
        Err(err) => match err {
            Error::NotFound => Err((StatusCode::NOT_FOUND, format!("The location with id: \"{id}\" doesn't exist"))),
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "Something unexpected happened".to_string()))
        }
    }
}

pub async fn get_all_location_list(State(pool): State<PoolType>) -> Json<Vec<LocationResponse>> {
    let locations = pool.get().await.unwrap().interact(move |conn| {
        // TODO: Optimise if empty
        let location_ids = LocationModel::list(conn).unwrap_or(vec![]);
        LocationModel::get_list(conn, location_ids).unwrap_or(vec![])
    }).await.unwrap();

    Json(locations.iter().map(|x| x.into()).collect())
}
