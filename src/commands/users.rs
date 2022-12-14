use std::sync::Arc;

use serenity::{
    builder::{CreateCommand, CreateInteractionResponseMessage},
    model::prelude::CommandInteraction,
};

use crate::app_state::AppState;

pub fn run(_: CommandInteraction, state: Arc<AppState>) -> CreateInteractionResponseMessage {
    let connected_users = state.users.lock().iter().filter(|x| x.1.connected).count();
    CreateInteractionResponseMessage::new().content(format!("Connected users: {}", connected_users))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("users").description("Get the number of connected users")
}
