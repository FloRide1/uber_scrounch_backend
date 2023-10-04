use std::{collections::HashMap, vec};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::result::Error;

use crate::{
    models::{command_model::{CommandModel, NewCommandModel}, command_products_model::{CommandProductModel, NewCommandProductModel}, user_model::UserModel},
    state::PoolType,
};

use super::{
    oauth::{admin::Admin, user::User},
    response::command_response::CommandResponse, request::command_request::{CommandRequest, CommandItemRequest},
};

pub async fn get_command(user: User, admin: Option<Admin>, Path(id): Path<i32>, State(pool): State<PoolType>) -> Result<CommandResponse, impl IntoResponse> {
    let res = pool.get().await.unwrap().interact(move |conn| {
        CommandModel::get(conn, id)?.into_response(conn)
    }).await.unwrap();


    match res {
        Ok(command) => {
            match admin.is_some() || command.user_id == user.id {
                true => Ok(command),
                false => Err((StatusCode::FORBIDDEN, "You're not authorized to view this command".to_string()))
            }
        },
        Err(err) => match  err {
            Error::NotFound => Err((StatusCode::NOT_FOUND, format!("The command with id: \"{id}\" doesn't exist"))),
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "Something unexpected happened".to_string()))

        }
    }
}

pub async fn get_all_commands(user: User, State(pool): State<PoolType>) -> Result<Json<Vec<CommandResponse>>, impl IntoResponse> {
    let res: Result<Vec<CommandResponse>, diesel::result::Error> = pool.get().await.unwrap().interact(move |conn| {
        let user = UserModel::get(conn, user.id)?; // It should never happened 
        let commands = user.get_commands(conn)?;
        let res = commands
            .iter()
            .map(|x| x.into_response(conn))
            .filter_map(|x| x.ok())
            .collect();
        Ok(res)
    }).await.unwrap();

    match res {
        Ok(res) => Ok(Json(res)),
        Err(err) => match err {
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "Something unexpected happened").into_response())
        }
    }
}

pub async fn post_command(user: User, State(pool): State<PoolType>, Json(command): Json<CommandRequest>) -> Response {
    let mut command = command;

    // Security
    if command.items.iter().any(|x| x.amount < 1) {
        return (StatusCode::BAD_REQUEST, "You can't have null or negative amount of product").into_response();
    }
    // TODO:: Add max product amount
    if command.items.iter().fold(0, |a, b| a + b.amount) > 6 {
        return (StatusCode::BAD_REQUEST, "You can't have more than 6 items").into_response();
    }

    // Merge items
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for item in command.items.iter() {
        let id = item.id;
        let amount = item.amount;
        match hash_map.get(&id) {
            None => hash_map.insert(id, amount),
            Some(x) => hash_map.insert(id, x + amount),
        };
    }
    command.items = hash_map.iter().map(|x| CommandItemRequest { id: *x.0, amount: *x.1 }).collect();
    //

    let res: Result<CommandResponse, diesel::result::Error> = pool.get().await.unwrap().interact(move |conn| {
        // TODO: Add check of product vs stock

        let new_command  = CommandModel::new(conn, NewCommandModel { user_id: user.id, location_id: command.location })?; 
        let command_products = command.items.iter()
            .map(|x| 
                 NewCommandProductModel {
                    product_id: x.id,
                    command_id: new_command.id,
                    amount: x.amount
                }
            ).collect();
        CommandProductModel::new_list(conn, command_products)?;
        new_command.into_response(conn)
    }).await.unwrap();

    match res {
        Ok(res) => { 
            if let Ok(url) = std::env::var("DISCORD_WEBHOOK") {
                let _ = discord_webhook_client::send_message(url::Url::parse(&url).unwrap(), &discord_message::DiscordMessage { 
                    username: None,
                    avatar_url: None,
                    content: format!("New Command (˵ ͡° ͜ʖ ͡°˵): {}", res).to_string(),
                    embeds: vec![]
                }).await;
            }
            info!("New Command: {}", res);


            (StatusCode::CREATED, Json(res)).into_response()
        },
        Err(err) => match err {
            // TODO : Handle error
            _ => { 
                error!("Command can't be created because: \"{:?}\"", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Something unexpected happened").into_response() 
            }
        }
    }
}
