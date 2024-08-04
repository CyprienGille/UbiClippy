use std::sync::Mutex;

use tauri::{
    CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

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

pub fn handle_tray_event(event: SystemTrayEvent, app: &tauri::AppHandle) {
    if let Some(main_window) = app.get_window("main") {
        match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                if let Err(e) = summon_window(&main_window) {
                    eprintln!("Error: {}", e);
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    if let Err(e) = summon_window(&main_window) {
                        eprintln!("Error: {}", e);
                    }
                }
                "hide" => {
                    if let Err(e) = hide_window(&main_window) {
                        eprintln!("Error: {}", e);
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        }
    } else {
        eprintln!("Could not find window 'main'.")
    }
}

pub struct HasLostFocusOnce {
    lost_focus: Mutex<bool>,
}

impl HasLostFocusOnce {
    pub fn new() -> Self {
        Self {
            lost_focus: Mutex::new(false),
        }
    }
}

#[tauri::command]
pub fn toggle_window(
    app: tauri::AppHandle,
    focus_state: State<HasLostFocusOnce>,
) -> Result<(), String> {
    match app.get_window("main") {
        Some(main_window) => match main_window.is_focused() {
            Ok(true) => {
                println!("Hiding window because it is focused");
                hide_window(&main_window)
            }
            Ok(false) => {
                if *focus_state.lost_focus.lock().unwrap() {
                    println!("Showing window because it is not focused");
                    summon_window(&main_window)
                } else {
                    println!("Hiding window because it is focused but has never lost focus");
                    *focus_state.lost_focus.lock().unwrap() = true;
                    hide_window(&main_window)
                }
            }
            Err(_) => Err("Could not figure out if window was focused.".to_string()),
        },
        None => Err("Could not find window 'main'.".to_string()),
    }
}

pub fn summon_window(main_window: &tauri::Window) -> Result<(), String> {
    if let Err(e) = main_window.show() {
        return Err(format!("Failed to show on window 'main': {}", e));
    }
    if let Err(e) = main_window.unminimize() {
        return Err(format!("Failed to maximize on window 'main': {}", e));
    }
    if let Err(e) = main_window.set_focus() {
        return Err(format!("Failed to focus on window 'main': {}", e));
    }
    Ok(())
}

pub fn hide_window(main_window: &tauri::Window) -> Result<(), String> {
    if let Err(e) = main_window.hide() {
        return Err(format!("Failed to hide window 'main': {}", e));
    }

    Ok(())
}
