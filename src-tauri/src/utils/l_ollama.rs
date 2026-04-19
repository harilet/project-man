use ollama_rs::coordinator::Coordinator;
use ollama_rs::{
    generation::chat::{ChatMessage, ChatMessageResponse},
    Ollama,
};
use std::error::Error;
use tauri::Url;

use crate::utils::db;
use crate::utils::ollama_tool;

fn split_long_messages(messages: Vec<ChatMessage>) -> Vec<ChatMessage> {
    const MAX_LENGTH: usize = 1000;
    const OVERLAP: usize = 100;

    let mut result = Vec::new();

    for message in messages {
        let content = message.content.clone();
        let total_chunk_count = content.len() / (MAX_LENGTH - OVERLAP);

        if content.len() <= MAX_LENGTH {
            result.push(message);
            continue;
        } else {
            // Split the content into chunks with overlap
            let mut start = 0;

            while start < content.len() {
                let end = (start + MAX_LENGTH).min(content.len());
                let chunk = content[start..end].to_string();

                let mut split_message = message.clone();
                split_message.content = format!(
                    "Chunk {}of{}\n{}",
                    end / (MAX_LENGTH - OVERLAP),
                    total_chunk_count,
                    chunk
                );
                result.push(split_message);

                // Move start position with overlap, but ensure we make progress
                if end >= content.len() {
                    break;
                }
                start = end.saturating_sub(OVERLAP);

                // Prevent infinite loop if overlap is too large
                if start <= start.saturating_sub(OVERLAP) && end < content.len() {
                    start = end;
                }
            }
        }
    }

    result
}

#[derive(Clone, serde::Serialize, Debug)]
struct ModelData {
    name: String,
    architecture: String,
    context: String,
    capabilities: Vec<String>,
}

pub(crate) async fn get_all_local_models() -> Result<Vec<String>, Box<dyn Error>> {
    let ollama_server = match db::get_ollama_setting().await?.get("ollama_server") {
        Some(da) => da.clone(),
        None => "".to_string(),
    };
    let ollama = Ollama::from_url(Url::parse(&ollama_server)?);
    let res = ollama.list_local_models().await?;
    let mut local_models: Vec<String> = vec![];
    for local_model in res {
        let model_info = ollama.show_model_info(local_model.name.clone()).await?;
        let architecture = match model_info.model_info.get("general.architecture") {
            Some(architecture) => architecture.to_string().replace("\"", ""),
            None => "".to_string(),
        };
        let context_key = format!("{}.context_length", architecture);

        let context_length = match model_info.model_info.get(&context_key) {
            Some(context) => match context.as_u64() {
                Some(num) => num,
                None => 0,
            },
            None => 0,
        };

        let model_data = ModelData {
            name: local_model.name,
            architecture: architecture,
            context: format!("{}", context_length),
            capabilities: model_info.capabilities,
        };
        local_models.push(serde_json::to_string(&model_data)?);
    }

    Ok(local_models)
}

pub(crate) async fn send_message(
    history: Vec<ChatMessage>,
    request: Vec<ChatMessage>,
) -> Result<ChatMessageResponse, Box<dyn Error>> {
    let ollama_settings = db::get_ollama_setting().await?;
    let ollama_server = ollama_settings
        .get("ollama_server")
        .cloned()
        .unwrap_or_default();
    let ollama_model = ollama_settings.get("model").cloned().unwrap_or_default();

    let ollama = Ollama::from_url(Url::parse(&ollama_server)?);

    // Split long messages in both history and request
    let split_history = split_long_messages(history);
    let split_request = split_long_messages(request);

    let response = Coordinator::new(ollama, ollama_model, split_history)
        .add_tool(ollama_tool::read_repo_file)
        .add_tool(ollama_tool::list_dir)
        .add_tool(ollama_tool::search_code)
        .add_tool(ollama_tool::read_multiple_files)
        .add_tool(ollama_tool::get_staged_diff)
        .chat(split_request)
        .await?;

    Ok(response)
}
