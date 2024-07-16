use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "PascalCase"))]
pub enum Role {
    #[serde(alias = "system")]
    System,
    #[default]
    #[serde(alias = "user")]
    User,
    #[serde(alias = "assistant")]
    Assistant,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    images: Option<Vec<String>>,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Message {
            role,
            content,
            images: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

impl ChatRequest {
    pub fn new(model: String, messages: Vec<Message>) -> Self {
        ChatRequest { model, messages }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ChatResponse {
    model: String,
    created_at: String,
    pub message: Option<Message>,
    pub done: bool,
    total_duration: Option<i64>,
    load_duration: Option<i64>,
    prompt_eval_count: Option<i64>,
    prompt_eval_duration: Option<i64>,
    eval_count: Option<i64>,
    eval_duration: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ModelDetails {
    format: String,
    family: String,
    families: Vec<String>,
    parameter_size: String,
    quantization_level: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ModelInfo {
    name: String,
    modified_at: String,
    size: i64,
    digest: String,
    details: ModelDetails,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct TagsResponse {
    models: Vec<ModelInfo>,
}

pub async fn chat_response(req: ChatRequest) -> Result<Response, reqwest::Error> {
    let client = Client::new();

    let res = client
        .post("http://localhost:11434/api/chat")
        .json(&req)
        .send()
        .await?;
    Ok(res)
}

pub async fn tags_response() -> Result<Response, reqwest::Error> {
    let client = Client::new();

    let res = client.get("http://localhost:11434/api/tags").send().await?;
    Ok(res)
}

#[tauri::command]
pub async fn get_all_models() -> Vec<ModelInfo> {
    match tags_response().await {
        Ok(resp) => {
            if let Ok(bytes) = resp.bytes().await {
                match serde_json::from_slice::<TagsResponse>(&bytes) {
                    Ok(all_tags) => all_tags.models,
                    Err(e) => {
                        println!("Malformated response: {}", e);
                        Vec::new()
                    }
                }
            } else {
                println!("Could not turn response into bytes.");
                Vec::new()
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            Vec::new()
        }
    }
}
