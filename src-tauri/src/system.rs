use tauri::Manager;
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

pub fn init_tray() -> tauri::SystemTray {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

#[tauri::command]
pub fn toggle_window(app: tauri::AppHandle) -> Result<(), String> {
    match app.get_window("main") {
        Some(main_window) => {
            if main_window.is_focused().unwrap_or_default() {
                hide_window(app)
            } else {
                summon_window(app)
            }
        }
        None => Err("Could not find window 'main'.".to_string()),
    }
}

pub fn summon_window(app: tauri::AppHandle) -> Result<(), String> {
    match app.get_window("main") {
        Some(main_window) => {
            if let Err(e) = main_window.show() {
                return Err(format!("Failed to show on window 'main': {}", e));
            }
            if let Err(e) = main_window.unminimize() {
                return Err(format!("Failed to maximize on window 'main': {}", e));
            }
            if let Err(e) = main_window.set_focus() {
                return Err(format!("Failed to focus on window 'main': {}", e));
            }
        }
        None => return Err("Could not find window 'main'.".to_string()),
    };
    Ok(())
}

pub fn hide_window(app: tauri::AppHandle) -> Result<(), String> {
    match app.get_window("main") {
        Some(main_window) => {
            if let Err(e) = main_window.hide() {
                return Err(format!("Failed to hide window 'main': {}", e));
            }
        }
        None => return Err("Could not find window 'main'.".to_string()),
    };
    Ok(())
}
