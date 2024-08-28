use std::{error::Error, fs::File, io::BufReader, path::Path};

use tauri::{AppHandle, State};

use crate::prompts::{Prompt, PromptLib};

const LIB_FILE_NAME: &str = "prompt_library.json";

#[tauri::command]
pub fn load_prompt_lib(all_prompts: State<PromptLib>, app: AppHandle) -> Result<(), String> {
    if let Some(data_dir) = app.path_resolver().app_data_dir() {
        match read_lib_from_file(data_dir.join(LIB_FILE_NAME)) {
            Ok(lib) => {
                *all_prompts.prompts.lock().unwrap() = lib;
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Error while looking for the app data dir".to_string())
    }
}

fn read_lib_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Prompt>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lib = serde_json::from_reader(reader)?;

    Ok(lib)
}
