use ollama_rs::{
    generation::chat::{
        request::ChatMessageRequest, ChatMessage, ChatMessageResponse, MessageRole,
    },
    Ollama,
};
use std::error::Error;
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
    model: String,
    messages: Vec<ChatMessage>,
    history: Vec<ChatMessage>,
) -> Result<ChatMessageResponse, Box<dyn Error>> {
    let ollama_server = match db::get_ollama_setting().await?.get("ollama_server") {
        Some(da) => da.clone(),
        None => "".to_string(),
    };

    let messsage_request = ChatMessageRequest::new(model, messages.clone());

    let mut t_history = history.clone();

    let mut ollama = Ollama::from_url(Url::parse(&ollama_server)?);

    let res: ChatMessageResponse = ollama
        .send_chat_messages_with_history(&mut t_history, messsage_request)
        .await?;
    Ok(res)
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

    let context_size = 3000;
    let context_overlap = 100;

    let mut file_summries = vec![];

    let legth = files.len();

    for (index, file) in files.iter().enumerate() {
        app.emit("commit-progress", format!("{}/{}", index, legth))
            .unwrap();
        let change_content = git::get_file_diff(location.clone(), file.clone())?;
        let mut messages = vec![];

        let main_content_message=format!(
            "You are a precise code summarizer that extracts key atomic changes from file diffs in a compact, structured format.

Your goal is to identify what changed and the observable reason behind it, based strictly on the diff content — no guessing or speculation.

Output Format (strict JSON):
{{
    \"file\": \"<filename>\",
    \"changes\": [
        {{\"action\": \"<verb>\", \"target\": \"<entity>\", \"reason\": \"<short purpose>\"}}
    ]
}}

Guidelines:
- Use one object per file.
- Each \"changes\" entry represents an atomic edit (e.g., a function, class, or config change).
- Use short, consistent action verbs:
  add, remove, update, fix, refactor, rename, move, change, etc.
- The \"target\" should be a concise identifier (e.g., function name, class, module, feature, config).
- The \"reason\" must describe the purpose in 3-8 words, inferred from context (e.g., \"to fix bug\", \"to simplify logic\", \"to support new feature\").
- Exclude irrelevant or trivial edits (formatting, spacing, comment-only changes).
- Output valid JSON only — no explanations, no Markdown, no prose.

filename: {}
changes:
{}",
            file,
            change_content.join("\n")
        );
        let contx_length = main_content_message.chars().count() / 4;

        if contx_length >= context_size {
            for i in 0..((contx_length / context_size) + 1) {
                let start = i * context_size;
                let end = ((i + 1) * context_size) + context_overlap;
                messages.push(ChatMessage::new(
                    MessageRole::User,
                    format!(
                        "Chunk {}/{}:\n<text>\n{}\n</text>",
                        i + 1,
                        (contx_length / context_size) + 1,
                        main_content_message[start..end].to_string()
                    ),
                ));
            }
        } else {
            messages.push(ChatMessage::new(MessageRole::User, main_content_message));
        }

        let mut cmr = ChatMessageRequest::new(model.clone(), messages);

        cmr = cmr.think(false);

        let response = ollama.send_chat_messages(cmr).await?;
        let mut response_content = response.message.content;
        if response_content.contains("```json") {
            response_content = response_content.replace("```json", "");
            response_content = response_content.replace("```", "");
        }
        file_summries.push(response_content.clone());

        app.emit(
            "file-summrie",
            format!(
                "{{\"file\":\"{}\",\"response\":{}}}",
                file, response_content
            ),
        )
        .unwrap();
        app.emit("commit-progress", format!("{}/{}", index + 1, legth))
            .unwrap();
    }

    let mut messages = vec![];

    let main_content_message = format!(
        "You are a professional Git commit message generator.
Your task is to write a single, concise commit message (one sentence) describing all changes across files.

Guidelines:
- Combine the provided atomic edits naturally into a coherent commit message.
- Capture both what changed and why, if implied.
- Use imperative tone (e.g., “Add…”, “Refactor…”, “Fix…”).
- Do not invent motivations or context not present in the edits.
- Output only the commit message — no extra text, formatting, or explanations.

File changes:
{}",
        file_summries.join("\n\n")
    );

    let contx_length = main_content_message.chars().count() / 4;

    if contx_length >= context_size {
        for i in 0..((contx_length / context_size) + 1) {
            let start = i * context_size;
            let end = ((i + 1) * context_size) + context_overlap;
            messages.push(ChatMessage::new(
                MessageRole::User,
                format!(
                    "Chunk {}/{}:\n<text>\n{}\n</text>",
                    i + 1,
                    (contx_length / context_size) + 1,
                    main_content_message[start..end].to_string()
                ),
            ));
        }
    } else {
        messages.push(ChatMessage::new(MessageRole::User, main_content_message));
    }

    app.emit("get-history", messages.clone()).unwrap();
    let cmr = ChatMessageRequest::new(model.clone(), messages);

    let response = ollama.send_chat_messages(cmr).await?;
    Ok(response)
}
