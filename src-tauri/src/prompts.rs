use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

pub struct IdState {
    next_id: Mutex<u64>,
}

impl IdState {
    fn consume_free_id(&self) -> u64 {
        let free_id = *self.next_id.lock().unwrap();
        *self.next_id.lock().unwrap() += 1;
        free_id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    id: u64,
    content: String,
    enabled: bool,
    trigger: Option<String>,
}

impl Prompt {
    fn new(id: u64, content: String, enabled: bool, trigger: Option<String>) -> Self {
        Prompt {
            id,
            content,
            enabled,
            trigger,
        }
    }
}

pub struct PromptLib {
    prompts: Mutex<Vec<Prompt>>,
}

impl PromptLib {
    pub fn new() -> Self {
        PromptLib {
            prompts: Mutex::new(Vec::new()),
        }
    }

    pub fn defaults() -> Self {
        PromptLib {
            prompts: Mutex::new(vec![
                Prompt::new(0, "Hi!".to_string(), true, Some("h".to_string())),
                Prompt::new(
                    1,
                    "Who are you? Answer in a single sentence.".to_string(),
                    true, 
                    Some("w".to_string())
                ),
                Prompt::new(
                    2,
                    "Summarize the following text in two sentences. Do not say 'Here is a summary of the text'.\n$CLIPBOARD$".to_string(),
                    true, 
                    Some("s".to_string())
                ),
                Prompt::new(
                    3,
                    "What is the following about?\n$CLIPBOARD$".to_string(),
                    true, 
                    None
                ),
                Prompt::new(
                    4,
                    "Format this nicely.\n$CLIPBOARD$".to_string(),
                    true, 
                    None
                ),
                Prompt::new(
                    5,
                    "What does this code do?.\n$CLIPBOARD$".to_string(),
                    true, 
                    None
                ),
                Prompt::new(
                    6,
                    "Is this my first message to you?".to_string(),
                    true, 
                    Some("i".to_string())
                )
            ]),
        }
    }
}

#[tauri::command]
pub fn add_prompt(
    new_prompt: String,
    trigger: Option<String>,
    all_prompts: State<PromptLib>,
    id_state: State<IdState>,
) {
    let prompt = Prompt::new(id_state.consume_free_id(), new_prompt, true, trigger);
    all_prompts.prompts.lock().unwrap().push(prompt);
}

#[tauri::command]
pub fn get_all_prompts(all_prompts: State<PromptLib>) -> Vec<Prompt> {
    (*all_prompts.prompts.lock().unwrap()).clone()
}
