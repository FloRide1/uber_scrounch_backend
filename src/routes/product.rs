use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::result::Error;

use crate::{models::product_model::ProductModel, state::PoolType};

use super::response::product_response::ProductResponse;

pub async fn get_product_ids(State(pool): State<PoolType>) -> Json<Vec<i32>> {
    let res = pool
        .get()
        .await
        .unwrap()
        .interact(move |conn| ProductModel::list(conn).unwrap_or_default())
        .await
        .unwrap();

    Json(res)
}

pub async fn get_product(
    Path(id): Path<i32>,
    State(pool): State<PoolType>,
) -> Result<ProductResponse, impl IntoResponse> {
    let res = pool
        .get()
        .await
        .unwrap()
        .interact(move |conn| ProductModel::get(conn, id))
        .await
        .unwrap();

    match res {
        Ok(res) => Ok(res.into()),
        Err(err) => match err {
            Error::NotFound => Err((
                StatusCode::NOT_FOUND,
                format!("The product with id: \"{id}\" doesn't exist"),
            )),
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something unexpected happened".to_string(),
            )),
        },
    }
}

pub async fn get_all_product_list(State(pool): State<PoolType>) -> Json<Vec<ProductResponse>> {
    let products = pool
        .get()
        .await
        .unwrap()
        .interact(move |conn| {
            // TODO: Optimise if empty
            let product_ids = ProductModel::list(conn).unwrap_or_default();
            ProductModel::get_list(conn, product_ids).unwrap_or_default()
        })
        .await
        .unwrap();

    Json(products.iter().map(|x| x.into()).collect())
}
