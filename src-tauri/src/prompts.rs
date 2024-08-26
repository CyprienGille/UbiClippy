use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

pub struct IdState {
    next_id: Mutex<u64>,
}

impl IdState {
    // pub fn new() -> Self {
    //     IdState { next_id: Mutex::new(0) }
    // }
    
    pub fn from_prompt_lib(pl: PromptLib) -> Self {
        let max_id = pl.prompts.lock().unwrap()
            .clone()
            .into_iter()
            .max_by_key(|prompt| prompt.id)
            .map(|prompt| prompt.id + 1)
            .unwrap_or(0);
        IdState {
            next_id: Mutex::new(max_id),
        }
    }

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
    // pub fn new() -> Self {
    //     PromptLib {
    //         prompts: Mutex::new(Vec::new()),
    //     }
    // }

    pub fn default() -> Self {
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
                    "Format this nicely.\n$CLIPBOARD$".to_string(),
                    true, 
                    None
                ),
                Prompt::new(
                    4,
                    "What does this code do?\n$CLIPBOARD$".to_string(),
                    true, 
                    None
                ),
                Prompt::new(
                    5,
                    "How many helicopters can a human eat in one sitting?".to_string(),
                    true, 
                    Some("i".to_string())
                ),
                Prompt::new(
                    6,
                    "What does the word '$CLIPBOARD$' mean?".to_string(),
                    true, 
                    Some("d".to_string())
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
pub fn toggle_prompt(
    id:u64,
    all_prompts: State<PromptLib>,
) {
    
    all_prompts.prompts
    .lock()
    .unwrap()
    .iter_mut()
    .filter(|prompt| prompt.id == id)
    .for_each(|prompt| prompt.enabled = !prompt.enabled);
}

#[tauri::command]
pub fn edit_prompt_content(
    id:u64,
    content:String,
    all_prompts: State<PromptLib>,
) {
    
    all_prompts.prompts
    .lock()
    .unwrap()
    .iter_mut()
    .filter(|prompt| prompt.id == id)
    .for_each(|prompt| prompt.content.clone_from(&content));
}

#[tauri::command]
pub fn remove_prompt(id: u64, all_prompts: State<PromptLib>) {
   let mut prompts = all_prompts.prompts.lock().unwrap();
   if let Some(pos) = prompts.iter().position(|prompt| prompt.id == id) {
       prompts.remove(pos);
   }
}


#[tauri::command]
pub fn get_all_prompts(all_prompts: State<PromptLib>) -> Vec<Prompt> {
    (*all_prompts.prompts.lock().unwrap()).clone()
}

