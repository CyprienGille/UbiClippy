use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    total_duration: Option<i64>,
    load_duration: Option<i64>,
    prompt_eval_count: Option<i64>,
    prompt_eval_duration: Option<i64>,
    eval_count: Option<i64>,
    eval_duration: Option<i64>,
    context: Option<Vec<i64>>,
}

#[derive(Serialize, Debug, Default)]
pub struct OllamaRequest {
    model: String,
    prompt: String,
}
