use tauri::State;
use crate::settings::{AppSettings, SettingsState, save_settings};

#[tauri::command]
pub fn get_settings(state: State<'_, SettingsState>) -> AppSettings {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
pub fn update_settings(state: State<'_, SettingsState>, settings: AppSettings) -> Result<AppSettings, String> {
    save_settings(&settings)?;
    let mut current = state.0.lock().unwrap();
    *current = settings.clone();
    Ok(settings)
}
