use crate::PasswordManager;

#[tauri::command(rename_all = "camelCase")]
/// Generate a password.
///
/// # Arguments
///
/// * `length` - The length of the password to generate.
///
/// # Returns
///
/// A Result containing the generated password or an error.
///
/// # Errors
///
/// If the password cannot be generated.
pub async fn generate_password(length: usize) -> Result<String, String> {
    PasswordManager::generate_password(length).map_err(|e| e.to_string())
}
