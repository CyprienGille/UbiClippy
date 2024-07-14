use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "PascalCase"))]
enum Role {
    System,
    #[default]
    User,
    Assistant,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Message {
    role: Role,
    content: String,
    images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Default)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct ChatResponse {
    model: String,
    created_at: String,
    message: Option<Message>,
    done: bool,
    total_duration: Option<i64>,
    load_duration: Option<i64>,
    prompt_eval_count: Option<i64>,
    prompt_eval_duration: Option<i64>,
    eval_count: Option<i64>,
    eval_duration: Option<i64>,
}

#[derive(Serialize, Debug, Default)]
pub struct GenRequest {
    model: String,
    prompt: String,
}

impl GenRequest {
    pub fn new(model: String, prompt: String) -> Self {
        GenRequest { model, prompt }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GenResponse {
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

pub async fn gen_response(req: GenRequest) -> Result<Response, reqwest::Error> {
    let client = Client::new();

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&req)
        .send()
        .await?;
    Ok(res)
}
