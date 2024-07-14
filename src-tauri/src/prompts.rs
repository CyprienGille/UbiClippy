use std::sync::Mutex;
use tauri::State;

pub struct PromptLib {
    prompts: Mutex<Vec<String>>,
}

impl PromptLib {
    pub fn new() -> Self {
        PromptLib {
            prompts: Mutex::new(Vec::new()),
        }
    }

    pub fn defaults() -> Self {
        PromptLib {
            prompts: Mutex::new(vec!["Hi!".to_string(), "Hello there!".to_string()]),
        }
    }
}

#[tauri::command]
pub fn add_prompt(new_prompt: String, all_prompts: State<PromptLib>) {
    all_prompts.prompts.lock().unwrap().push(new_prompt);
}
