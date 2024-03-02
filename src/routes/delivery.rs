use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::result::Error;

use crate::{models::delivery_model::DeliveryModel, state::PoolType};

use super::response::delivery_response::DeliveryResponse;

pub async fn get_next_delivery(
    State(pool): State<PoolType>,
) -> Result<DeliveryResponse, impl IntoResponse> {
    let res = pool
        .get()
        .await
        .unwrap()
        .interact(move |conn| DeliveryModel::get_next(conn))
        .await
        .unwrap();

    match res {
        Ok(res) => Ok(res.into()),
        Err(err) => {
            error!("Something unexpected happened: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something unexpected happened".to_string(),
            ))
        }
    }
}

pub async fn get_delivery(
    Path(id): Path<i32>,
    State(pool): State<PoolType>,
) -> Result<DeliveryResponse, impl IntoResponse> {
    let res = pool
        .get()
        .await
        .unwrap()
        .interact(move |conn| DeliveryModel::get(conn, id))
        .await
        .unwrap();

    match res {
        Ok(res) => Ok(res.into()),
        Err(err) => match err {
            Error::NotFound => Err((
                StatusCode::NOT_FOUND,
                format!("The delivery with id: \"{id}\" doesn't exist"),
            )),
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something unexpected happened".to_string(),
            )),
        },
    }
}

pub async fn get_all_delivery_list(State(pool): State<PoolType>) -> Json<Vec<DeliveryResponse>> {
    let deliverys = pool
        .get()
        .await
        .unwrap()
        .interact(move |conn| DeliveryModel::get_futures(conn).unwrap_or(vec![]))
        .await
        .unwrap();

    Json(deliverys.iter().map(|x| x.into()).collect())
}
