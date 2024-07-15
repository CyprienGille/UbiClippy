use arboard::Clipboard;

#[tauri::command]
pub fn get_clipboard() -> String {
    if let Ok(mut cb) = Clipboard::new() {
        cb.get_text().unwrap_or_default()
    } else {
        "".to_string()
    }
}

pub fn replace_with_clipboard(text: String) -> String {
    let cb = get_clipboard();
    text.replace("$CLIPBOARD$", &cb)
}

#[tauri::command]
pub fn set_clipboard(content: &str) {
    match Clipboard::new() {
        Ok(mut cb) => {
            if let Err(e) = cb.set_text(content) {
                eprintln!("Failed to write to the clipboard: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to access the clipboard: {}", e),
    }
}
