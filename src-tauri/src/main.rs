// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::Manager;
use tauri::State;
use tauri::SystemTrayEvent;

mod clipboard;
mod ollama;
mod prompts;
mod system;

#[derive(Debug)]
struct CurrentChat {
    history: Mutex<Vec<ollama::Message>>,
}

#[tauri::command]
async fn process_chat(
    mut user_chat: String,
    model: String,
    app: tauri::AppHandle,
    chat: State<'_, CurrentChat>,
) -> Result<(), ()> {
    if user_chat.contains("$CLIPBOARD$") {
        user_chat = clipboard::replace_with_clipboard(user_chat);
    }

    add_to_chat(user_chat, ollama::Role::User, &chat);

    let current_history = chat.history.lock().unwrap().to_vec();

    if let Ok(new_msg) = chat_chunks(ollama::ChatRequest::new(model, current_history), app).await {
        add_to_chat(new_msg, ollama::Role::Assistant, &chat);
    }
    Ok(())
}

async fn chat_chunks(request: ollama::ChatRequest, app: tauri::AppHandle) -> Result<String, ()> {
    let mut new_msg = String::new();

    match ollama::chat_response(request).await {
        Ok(mut resp) => {
            while let Ok(Some(chunk)) = resp.chunk().await {
                match serde_json::from_slice::<ollama::ChatResponse>(&chunk) {
                    Ok(resp_data) => {
                        if let Some(ref msg) = resp_data.message {
                            new_msg += &msg.content;
                        }
                        app.emit_all("llm_chunk", resp_data).unwrap();
                    }
                    Err(e) => println!("Malformated response: {}", e),
                };
            }
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }
    Ok(new_msg)
}

#[tauri::command]
fn clear_chat(chat: State<CurrentChat>) {
    chat.history.lock().unwrap().clear();
}

fn add_to_chat(new_msg: String, role: ollama::Role, chat: &State<CurrentChat>) {
    chat.history
        .lock()
        .unwrap()
        .push(ollama::Message::new(role, new_msg));
}

fn main() {
    tauri::Builder::default()
        .manage(CurrentChat {
            history: Mutex::new(Vec::new()),
        })
        .manage(prompts::PromptLib::defaults())
        .invoke_handler(tauri::generate_handler![
            process_chat,
            clear_chat,
            clipboard::set_clipboard,
            clipboard::get_clipboard,
            ollama::get_all_models,
            prompts::add_prompt,
            prompts::get_all_prompts,
            system::summon_window,
            system::hide_window,
        ])
        .system_tray(system::init_tray())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                if let Err(e) = system::summon_window(app.clone()) {
                    eprintln!("Error: {}", e);
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "hide" => {
                    if let Err(e) = system::hide_window(app.clone()) {
                        eprintln!("Error: {}", e);
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
