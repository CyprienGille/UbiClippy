interface Message {
    role: string;
    content: string;
}

interface ChatResponse {
    message: Message;
    done: boolean;
}

interface LLMChunk {
    payload: ChatResponse;
}

interface Prompt {
    id: number;
    content: string;
    enabled: boolean;
    trigger: string,
}

interface ModelDetails {
    format: string;
    family: string;
    families: string;
    parameter_size: string;
    quantization_level: string;
}

interface ModelInfo {
    name: string;
    modified_at: string;
    size: number;
    details: ModelDetails;
}