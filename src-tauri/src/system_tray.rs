use tauri::Manager;

#[tauri::command]
pub async fn summon_window(app: tauri::AppHandle) -> Result<(), String> {
    match app.get_window("main") {
        Some(main_window) => {
            if let Err(e) = main_window.set_focus() {
                return Err(format!("Failed to focus on window 'main': {}", e));
            }
        }
        None => return Err("Could not find window 'main'.".to_string()),
    };
    Ok(())
}
