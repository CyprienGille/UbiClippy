// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::Manager;
use tauri::State;

mod clipboard;
mod ollama;

#[derive(Debug)]
struct CurrentChat {
    history: Mutex<Vec<ollama::Message>>,
}

#[tauri::command]
fn to_clipboard(text: String) {
    clipboard::set_clipboard(&text);
}

#[tauri::command]
async fn prompt_with_cb(app: tauri::AppHandle, model: String) {
    let cb_contents = clipboard::get_clipboard();
    let request = ollama::GenRequest::new(
        model,
        format!(
            "Summarize the text between <p> tags in two sentences.\
         Do not say 'Here is a summary of the text'.\
         \n <p>{}</p>",
            cb_contents
        ),
    );
    generate_response(app, request).await;
}

async fn generate_response(app: tauri::AppHandle, request: ollama::GenRequest) {
    match ollama::gen_response(request).await {
        Ok(mut resp) => {
            while let Ok(Some(chunk)) = resp.chunk().await {
                match serde_json::from_slice::<ollama::GenResponse>(&chunk) {
                    Ok(resp_data) => app.emit_all("llm_chunk", resp_data).unwrap(),
                    Err(e) => println!("Malformated response: {}", e),
                };
            }
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}

#[tauri::command]
fn clear_chat(chat: State<CurrentChat>) {
    chat.history.lock().unwrap().clear();
}

fn main() {
    tauri::Builder::default()
        .manage(CurrentChat {
            history: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            clear_chat,
            prompt_with_cb,
            to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
