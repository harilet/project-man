use ollama_rs::{
    coordinator::Coordinator,
    generation::chat::{
        request::ChatMessageRequest, ChatMessage, ChatMessageResponse, MessageRole,
    },
    Ollama,
};
use std::{error::Error, fs};
use tauri::{Emitter, Url};

use crate::{
    utils::{db, git},
    APP_HANDLE,
};

#[derive(Clone, serde::Serialize, Debug)]
struct ModelData {
    name: String,
    architecture: String,
    context: String,
    capabilities: Vec<String>,
}

#[derive(Clone, serde::Serialize, Debug)]
struct LlmToolCall {
    tool_name: String,
    tool_input: String,
    tool_output: String,
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
            Some(architecture) => architecture.to_string(),
            None => "".to_string(),
        };
        let model_data = ModelData {
            name: local_model.name,
            architecture: architecture,
            context: "12k".to_string(),
            capabilities: model_info.capabilities,
        };
        local_models.push(serde_json::to_string(&model_data)?);
    }

    Ok(local_models)
}

pub(crate) async fn send_message(
    model: String,
    messages: Vec<ChatMessage>,
    history: Vec<ChatMessage>,
) -> Result<ChatMessageResponse, Box<dyn Error>> {
    let ollama_server = match db::get_ollama_setting().await?.get("ollama_server") {
        Some(da) => da.clone(),
        None => "".to_string(),
    };
    let ollama = Ollama::from_url(Url::parse(&ollama_server)?);

    let mut coordinator = Coordinator::new(ollama, model.clone(), history.clone())
        .add_tool(get_file)
        .add_tool(get_file_diff);
    let res: ChatMessageResponse = coordinator.chat(messages.to_owned()).await?;
    Ok(res)
}

/// Get file contents from a file path.
///
/// * file_path - The file path to read from.
#[ollama_rs::function]
async fn get_file(file_path: String) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    println!("get_file: {file_path}");
    let file_contents = fs::read_to_string(&file_path)?;
    let app = APP_HANDLE.get().unwrap();
    app.emit(
        "tool-call",
        LlmToolCall {
            tool_name: "get_file".to_string(),
            tool_input: file_path,
            tool_output: file_contents.clone(),
        },
    )
    .unwrap();
    Ok(file_contents)
}

/// Get change diff of a file from a file path.
///
/// * file_path - The file path to read from(relative path).
/// * repo - The current repo we are using(absolute path).
///
/// Response format is each line is a json string key and values and have the following keys
/// `change_type` for the type of change that is as follows '-' are removed lines, '+' are added lines, ' ' do not have any change, 'M' indicates the file in content is modified, 'A' indicates the file in content is a new file, 'D' indicates the file in content is deleted file and 'H' the header for a change chunk",
/// `from_no` is the original line number,
/// `to_no` is the new line number and
/// `content` is the changes made
#[ollama_rs::function]
async fn get_file_diff(
    file_path: String,
    repo: String,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    println!("get_file_diff: {repo}::{file_path}");
    let file_contents = git::get_file_diff(repo.clone(), file_path.clone()).unwrap();
    let contents = file_contents.join("\n");
    println!("{}", contents);
    let app = APP_HANDLE.get().unwrap();
    app.emit(
        "tool-call",
        LlmToolCall {
            tool_name: "get_file_diff".to_string(),
            tool_input: format!("{repo}/{file_path}"),
            tool_output: contents.clone(),
        },
    )
    .unwrap();

    Ok(contents)
}

pub(crate) async fn generate_commit_message(
    location: String,
) -> Result<ChatMessageResponse, Box<dyn Error>> {
    let ollama_settings = db::get_ollama_setting().await?;
    let ollama_server = match ollama_settings.get("ollama_server") {
        Some(da) => da.clone(),
        None => "".to_string(),
    };
    let model = match ollama_settings.get("model") {
        Some(da) => da.clone(),
        None => "".to_string(),
    };

    let files = git::get_staged_files(location.clone())?;

    let ollama = Ollama::from_url(Url::parse(&ollama_server)?);
    let app = APP_HANDLE.get().unwrap();

    let mut file_summries = vec![];

    let legth = files.len();

    for (index, file) in files.iter().enumerate() {
        app.emit("commit-progress", format!("{}/{}", index, legth))
            .unwrap();
        let change_content = git::get_file_diff(location.clone(), file.clone())?;
        let mut messages = vec![];
        messages.push(ChatMessage::new(
            MessageRole::System,
            "You are a programming expert who generates precise and unambiguous responses."
                .to_string(),
        ));
        messages.push(ChatMessage::new(
            MessageRole::User,
            format!(
                "`change_type` for the type of change that is as follows '-' are removed lines, '+' are added lines, ' ' do not have any change, 'M' indicates the file in content is modified, 'A' indicates the file in content is a new file, 'D' indicates the file in content is deleted file and 'H' the header for a change chunk
 `from_no` is the original line number,
 `to_no` is the new line number and
 `content` is the changes made
 create a brief and concise summrize the following file changes:\nfilename: {}\nchanges:\n{}",
                file,
                change_content.join("\n")
            ),
        ));

        println!("{:#?}", messages);

        let mut cmr = ChatMessageRequest::new(model.clone(), messages);

        cmr = cmr.think(false);

        let response = ollama.send_chat_messages(cmr).await?;

        println!("\n{}\n", response.message.content);

        file_summries.push(response.message.content);
        app.emit("commit-progress", format!("{}/{}", index + 1, legth))
            .unwrap();
    }

    let mut messages = vec![];
    messages.push(ChatMessage::new(
        MessageRole::System,
        "You are a programming expert who generates precise and unambiguous responses.".to_string(),
    ));
    messages.push(ChatMessage::new(
        MessageRole::User,
        format!(
            "Reply with a single commit message (1 sentence, covering all files), including both what changed and why it was done.

Do not invent details.
Do not provide explanations beyond the commit message or the question.

The Following is the file changes:\n{}",
            file_summries.join("\n\n")
        ),
    ));

    app.emit("get-history", messages.clone()).unwrap();
    let cmr = ChatMessageRequest::new(model.clone(), messages);

    let response = ollama.send_chat_messages(cmr).await?;
    println!("{}", response.message.content);
    Ok(response)
}
