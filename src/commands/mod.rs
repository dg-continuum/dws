use std::sync::Arc;

use once_cell::sync::Lazy;
use serenity::{builder::*, http::Http, model::application::interaction::application_command::*};

use crate::{app_state::AppState, config::CONFIG, Result};

mod change_perms;
mod irc;
mod link;
mod users;

pub static REST: Lazy<Http> = Lazy::new(|| {
    let http = Http::new(&CONFIG.discord_token);
    http.set_application_id(CONFIG.discord_client_id);
    http
});

pub async fn handle_command(interaction: CommandInteraction, state: Arc<AppState>) -> CreateInteractionResponse {
    let roles = interaction.member.clone().map(|x| x.roles).unwrap();
    let admin = match CONFIG.discord_admin_role {
        Some(role) => roles.contains(&role),
        None => false,
    };
    let res = match (interaction.data.name.as_str(), admin) {
        ("users", _) => users::run(interaction, state),
        ("change_perms", true) => change_perms::run(interaction, state),
        ("irc", _) => irc::run(interaction, state, admin).await,
        ("link", _) => link::run(interaction, state).await,
        _ => CreateInteractionResponseMessage::new().content("404 command not found lol".to_string()),
    };
    CreateInteractionResponse::Message(res)
}
pub async fn register() -> Result<()> {
    REST.create_global_application_commands(&vec![
        users::register(),
        change_perms::register(),
        irc::register(),
        link::register(),
    ])
    .await?;
    Ok(())
}
