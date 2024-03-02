use std::time::{Duration, SystemTime};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::result::Error;

use crate::{
    models::delivery_model::{DeliveryModel, NewDeliveryModel},
    state::PoolType,
};

use super::{
    oauth::admin::Admin, request::delivery_request::DeliveryRequest,
    response::delivery_response::DeliveryResponse,
};

pub async fn get_next_delivery(
    State(pool): State<PoolType>,
) -> Result<DeliveryResponse, impl IntoResponse> {
    let res = pool
        .get()
        .await
        .unwrap()
        .interact(DeliveryModel::get_next)
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
    let deliveries = pool
        .get()
        .await
        .unwrap()
        .interact(move |conn| DeliveryModel::get_futures(conn).unwrap_or_default())
        .await
        .unwrap();

    Json(deliveries.iter().map(|x| x.into()).collect())
}

pub async fn post_delivery(
    _admin: Admin,
    State(pool): State<PoolType>,
    Json(delivery): Json<DeliveryRequest>,
) -> impl IntoResponse {
    let time = SystemTime::UNIX_EPOCH + Duration::from_millis(delivery.time as u64);
    let res = pool
        .get()
        .await
        .unwrap()
        .interact(move |conn| DeliveryModel::new(conn, NewDeliveryModel { time }))
        .await
        .unwrap();

    match res {
        Ok(_) => (StatusCode::CREATED, "The delivery is create"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something unexpected happened",
        ),
    }
}
