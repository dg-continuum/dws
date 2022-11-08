#![allow(clippy::single_match)]

use std::{collections::HashSet, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};
use futures_util::future::join;
use parking_lot::Mutex;
use tokio::sync::broadcast as tokio_broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::*,
    app_state::AppState,
    commands::register,
    error::Result,
    messages::InternalMessages,
    utils::{
        retrieve_cosmetics::{retrieve_cosmetics, CosmeticFile, COSMETIC_FILE},
        set_ctrlc,
    },
};

pub mod app_state;
pub mod bitflags;
pub mod commands;
pub mod error;
pub mod messages;
pub mod utils;

mod api;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "dws=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    register().await?;
    let user_set = Mutex::new(HashSet::new());
    let (tx, mut rx) = tokio_broadcast::channel::<InternalMessages>(100);

    let cosmetics = retrieve_cosmetics().await;

    let app_state = Arc::new(AppState {
        user_set,
        tx: tx.clone(),
        broadcast_secret: std::env::var("BROADCAST_SECRET").unwrap_or_else(|_| "secret".into()),
        cosmetics: Mutex::new(cosmetics.cosmetics),
        users: Mutex::new(cosmetics.users),
    });

    set_ctrlc(app_state.clone())?;

    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));

        loop {
            interval.tick().await;

            let file = CosmeticFile {
                cosmetics: app_state_clone.cosmetics.lock().clone(),
                users: app_state_clone.users.lock().clone(),
            };
            tokio::fs::write(COSMETIC_FILE.as_str(), serde_json::to_string_pretty(&file).unwrap())
                .await
                .expect("Failed to write cosmetics file");
        }
    });

    // build our application with some routes
    let app = Router::with_state(app_state.clone())
        .route("/metrics", get(metrics::metrics))
        .route("/broadcast", post(broadcast::broadcast))
        .route("/cosmetics", get(cosmetics::cosmetics))
        .route("/discord", post(discord::handle_request))
        .route("/ws", get(ws::ws_handler));
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    // run it with hyper
    let addr = format!("{host}:{port}").parse().unwrap();
    tracing::debug!("listening on {}", addr);
    let (r, _) = join(axum::Server::bind(&addr).serve(app.into_make_service()), async {
        while let Ok(msg) = rx.recv().await {
            match msg {
                InternalMessages::RequestUser {
                    user_id,
                    requester_id,
                    nonce,
                } => {
                    let user_set = app_state.user_set.lock();
                    let is_online = user_set.contains(&user_id);
                    let msg = InternalMessages::UserRequestResponse {
                        is_online,
                        requester_id,
                        user_id,
                        nonce,
                    };
                    let _ = tx.send(msg);
                }
                InternalMessages::RequestUsersBulk {
                    user_ids,
                    requester_id,
                    nonce,
                } => {
                    let user_set = app_state.user_set.lock();
                    let list = user_ids
                        .into_iter()
                        .map(|user_id| {
                            let is_online = user_set.contains(&user_id);
                            (user_id, is_online)
                        })
                        .collect();
                    let msg = InternalMessages::UserRequestBulkResponse {
                        users: list,
                        requester_id,
                        nonce,
                    };
                    let _ = tx.send(msg);
                }
                _ => {}
            }
        }
    })
    .await;
    r?;
    Ok(())
}
