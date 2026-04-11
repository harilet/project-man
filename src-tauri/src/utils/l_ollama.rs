use ollama_rs::coordinator::Coordinator;
use ollama_rs::{
    generation::chat::{ChatMessage, ChatMessageResponse},
    Ollama,
};
use std::error::Error;
use tauri::Url;

use crate::utils::db;
use crate::utils::ollama_tool;

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

    let response = Coordinator::new(ollama, ollama_model, history)
        .add_tool(ollama_tool::read_repo_file)
        .add_tool(ollama_tool::list_dir)
        .add_tool(ollama_tool::search_code)
        .add_tool(ollama_tool::read_multiple_files)
        .add_tool(ollama_tool::get_staged_diff)
        .chat(request)
        .await?;

    Ok(response)
}
