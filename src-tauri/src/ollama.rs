use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct OllamaResponse {
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

impl OllamaRequest {
    pub fn new(model: String, prompt: String) -> Self {
        OllamaRequest { model, prompt }
    }
}

pub async fn get_response(req: OllamaRequest) -> Result<Response, reqwest::Error> {
    let client = Client::new();

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&req)
        .send()
        .await?;
    Ok(res)
}
