use log::{error, info};
use serde::Serialize;
use tauri::State;

use super::PasswordManagerState;

#[derive(Serialize)]
pub struct PasswordHealthResponse {
    pub service: String,
    pub username: String,
    pub score: u8,
    pub strength: String,
    pub issues: Vec<String>,
    pub suggestions: Vec<String>,
    pub last_modified: String,
}

#[tauri::command]
pub async fn check_passwords(
    state: State<'_, PasswordManagerState>,
) -> Result<Vec<PasswordHealthResponse>, String> {
    info!("Checking passwords");
    let state = state.0.lock().unwrap();
    match state.as_ref() {
        Some(pm) => match pm.check_passwords_health() {
            Ok(passwords) => {
                info!("Successfully checked passwords");
                let mut password_responses = Vec::new();
                for password in &passwords {
                    let response = PasswordHealthResponse {
                        service: password.service.clone(),
                        username: password.username.clone(),
                        score: password.score,
                        strength: format!("{:?}", password.strength),
                        issues: password.issues.iter().map(|i| format!("{:?}", i)).collect(),
                        suggestions: password.suggestions.clone(),
                        last_modified: password.last_modified.to_rfc3339(),
                    };
                    password_responses.push(response);
                }
                Ok(password_responses)
            }
            Err(e) => {
                error!("Failed to check passwords: {}", e);
                Err(e.to_string())
            }
        },
        None => {
            error!("Attempted to check passwords without being logged in");
            Err("Not logged in".into())
        }
    }
}
