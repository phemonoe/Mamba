use tauri::{command, Result as TauriResult};
use serde::{Deserialize, Serialize};
use anyhow;

mod poker_analyzer;
mod screenshot;

#[derive(Debug, Serialize, Deserialize)]
pub struct PokerAnalysis {
    pub action_recommendation: String,
    pub reasoning: String,
    pub hand_strength: String,
    pub pot_odds: Option<String>,
    pub confidence: f32,
}

#[command]
async fn analyze_poker_screenshot() -> TauriResult<PokerAnalysis> {
    match poker_analyzer::capture_and_analyze().await {
        Ok(analysis) => Ok(analysis),
        Err(e) => {
            log::error!("Failed to analyze poker screenshot: {}", e);
            Err(tauri::Error::Anyhow(anyhow::anyhow!(e.to_string())))
        }
    }
}

#[command]
async fn take_screenshot() -> TauriResult<String> {
    match screenshot::capture_screen().await {
        Ok(base64_image) => Ok(base64_image),
        Err(e) => {
            log::error!("Failed to take screenshot: {}", e);
            Err(tauri::Error::Anyhow(anyhow::anyhow!(e.to_string())))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        analyze_poker_screenshot,
        take_screenshot
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
