// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod clipboard;
mod ollama;

#[tauri::command]
fn response_to_clipboard(response: String) {
    clipboard::set_clipboard(&response);
}

#[tauri::command]
async fn prompt_with_cb(app: tauri::AppHandle, model: String) {
    let cb_contents = clipboard::get_clipboard();
    let request = ollama::OllamaRequest::new(
        model,
        format!(
            "Summarize the text between <p> tags in two sentences.\
         Do not say 'Here is a summary of the text'.\
         \n <p>{}</p>",
            cb_contents
        ),
    );
    trigger_response(app, request).await;
}

async fn trigger_response(app: tauri::AppHandle, request: ollama::OllamaRequest) {
    match ollama::get_response(request).await {
        Ok(mut resp) => {
            while let Ok(Some(chunk)) = resp.chunk().await {
                match serde_json::from_slice::<ollama::OllamaResponse>(&chunk) {
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            prompt_with_cb,
            response_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
