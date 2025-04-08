use serde::{Deserialize, Serialize};

use crate::models::ModelOptions;

use super::ChatMessage;

fn default_true() -> bool {
    true
}

/// A chat message request to Ollama.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageRequest {
    #[serde(rename = "model")]
    pub model_name: String,
    pub messages: Vec<ChatMessage>,
    // #[serde(skip_serializing_if = "Vec::is_empty")]
    // pub tools: Vec<ToolInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ModelOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub format: Option<FormatType>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_true")]
    pub stream: bool,
}

impl ChatMessageRequest {
    pub fn new(model_name: String, messages: Vec<ChatMessage>) -> Self {
        Self {
            model_name,
            messages,
            options: None,
            template: None,
            // format: None,
            // keep_alive: None,
            // Stream value will be overwritten by Ollama::send_chat_messages_stream() and Ollama::send_chat_messages() methods
            stream: false,
            // tools: vec![],
        }
    }

    /// Additional model parameters listed in the documentation for the Modelfile
    pub fn options(mut self, options: ModelOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// The full prompt or prompt template (overrides what is defined in the Modelfile)
    pub fn template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    // /// The format to return a response in.
    // pub fn format(mut self, format: FormatType) -> Self {
    //     self.format = Some(format);
    //     self
    // }

    // /// Used to control how long a model stays loaded in memory, by default models are unloaded after 5 minutes of inactivity
    // pub fn keep_alive(mut self, keep_alive: KeepAlive) -> Self {
    //     self.keep_alive = Some(keep_alive);
    //     self
    // }

    // /// Tools that are available to the LLM.
    // pub fn tools(mut self, tools: Vec<ToolInfo>) -> Self {
    //     self.tools = tools;
    //     self
    // }
}
