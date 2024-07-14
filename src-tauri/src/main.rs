// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod ollama;

#[tauri::command]
async fn trigger_response(app: tauri::AppHandle) {
    let request = ollama::OllamaRequest::new(
        "llama3:instruct".to_owned(),
        "Who are you? Respond in one to two sentences.".to_owned(),
    );

    match ollama::get_response(request).await {
        Ok(mut resp) => {
            while let Ok(Some(chunk)) = resp.chunk().await {
                match serde_json::from_slice::<ollama::OllamaResponse>(&chunk) {
                    Ok(resp_data) => app.emit_all("ollama_chunk", resp_data).unwrap(),
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
        .invoke_handler(tauri::generate_handler![trigger_response])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
