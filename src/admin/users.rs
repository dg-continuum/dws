use std::{collections::HashMap, sync::Arc};

use axum::extract::{Json, Query, State};
use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use serenity::model::prelude::UserId;
use uuid::Uuid;

use crate::{
    app_state::{AppState, User},
    bitflags::CosmeticFlags,
    error::Result,
    utils::{uuid_to_username, UuidAndUsername},
};

#[derive(Deserialize)]
pub struct AddUser {
    pub uuid: Uuid,
    pub linked_discord: Option<UserId>,
    pub enabled_prefix: Option<u8>,
    pub irc_blacklisted: Option<bool>,
    pub flags: Option<CosmeticFlags>,
}

#[derive(Deserialize)]
pub struct DeleteUser {
    pub uuid: Uuid,
}

pub async fn uuids_to_usernames(Json(uuids): Json<Vec<Uuid>>) -> Json<Vec<UuidAndUsername>> {
    let results = FuturesUnordered::new();
    for uuid in uuids {
        results.push(uuid_to_username(uuid));
    }
    Json(
        results
            .collect::<Vec<Result<UuidAndUsername>>>()
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect(),
    )
}

pub async fn get_users(State(state): State<Arc<AppState>>) -> Json<HashMap<Uuid, User>> {
    let users = state.users.lock();
    Json(users.clone())
}

pub async fn add_user(State(state): State<Arc<AppState>>, Json(data): Json<AddUser>) -> &'static str {
    let mut users = state.users.lock();
    let def = users.get(&data.uuid).cloned().unwrap_or_default();
    users.insert(
        data.uuid,
        User {
            linked_discord: data.linked_discord.or(def.linked_discord),
            enabled_prefix: data.enabled_prefix.or(def.enabled_prefix),
            irc_blacklisted: data.irc_blacklisted.unwrap_or(def.irc_blacklisted),
            flags: data.flags.unwrap_or(def.flags),
            ..def
        },
    );
    "ok"
}
pub async fn remove_user(State(state): State<Arc<AppState>>, Query(data): Query<DeleteUser>) -> &'static str {
    let mut users = state.users.lock();
    users.remove(&data.uuid);
    "ok"
}
