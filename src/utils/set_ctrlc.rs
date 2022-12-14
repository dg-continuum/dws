use std::sync::Arc;

use crate::{app_state::AppState, config::CONFIG, utils::retrieve_cosmetics::CosmeticFile};

pub fn set_ctrlc(app_state_clone: Arc<AppState>) -> crate::Result<()> {
    ctrlc::set_handler(move || {
        let file = CosmeticFile {
            cosmetics: app_state_clone.cosmetics.lock().clone(),
            users: app_state_clone.users.lock().clone(),
        };
        tracing::info!("Exiting...");
        std::fs::write(&CONFIG.cosmetics_file, serde_json::to_string_pretty(&file).unwrap())
            .expect("Failed to write cosmetics file");
        tracing::info!("Cosmetics file written");
        std::process::exit(0);
    })?;
    Ok(())
}
