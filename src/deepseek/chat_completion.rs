use serde::{Deserialize, Serialize};

use crate::deepseek::Model;

pub const ENDPOINT: &str = "/chat/completions";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: Model,
    pub messages: Vec<ChatMessage>,
    // enabled if not set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// [0, 2], default 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// [0, 1], default 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    // [0, 20]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl Request {
    pub fn builder(model: Model) -> RequestBuilder {
        RequestBuilder::new(model)
    }
}

pub struct RequestBuilder {
    model: Model,
    messages: Vec<ChatMessage>,
    thinking: Option<Thinking>,
    reasoning_effort: Option<ReasoningEffort>,
    max_tokens: Option<u32>,
    response_format: Option<ResponseFormat>,
    stop: Option<StopOption>,
    stream: Option<bool>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    tools: Vec<Tool>,
    tool_choice: Option<ToolChoice>,
    logprobs: Option<bool>,
    top_logprobs: Option<u32>,
    user_id: Option<String>,
}

impl RequestBuilder {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            messages: Vec::new(),
            thinking: None,
            reasoning_effort: None,
            max_tokens: None,
            response_format: None,
            stop: None,
            stream: None,
            temperature: None,
            top_p: None,
            tools: Vec::new(),
            tool_choice: None,
            logprobs: None,
            top_logprobs: None,
            user_id: None,
        }
    }

    pub fn add_message(mut self, message: ChatMessage) -> Self {
        self.messages.push(message);
        self
    }

    pub fn messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.messages = messages;
        self
    }

    pub fn thinking(mut self, thinking: Thinking) -> Self {
        self.thinking = Some(thinking);
        self
    }

    pub fn reasoning_effort(mut self, reasoning_effort: ReasoningEffort) -> Self {
        self.reasoning_effort = Some(reasoning_effort);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn stop(mut self, stop: StopOption) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn add_tool(mut self, tool: Tool) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = tools;
        self
    }

    pub fn tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    pub fn logprobs(mut self, logprobs: bool) -> Self {
        self.logprobs = Some(logprobs);
        self
    }

    pub fn top_logprobs(mut self, top_logprobs: u32) -> Self {
        self.top_logprobs = Some(top_logprobs);
        self
    }

    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn build(self) -> Request {
        Request {
            model: self.model,
            messages: self.messages,
            thinking: self.thinking,
            reasoning_effort: self.reasoning_effort,
            max_tokens: self.max_tokens,
            response_format: self.response_format,
            stop: self.stop,
            stream: self.stream,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            user_id: self.user_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum ChatMessage {
    System {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    User {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Assistant {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        // beta feature
        #[serde(skip_serializing_if = "Option::is_none")]
        prefix: Option<bool>,
        // beta feature
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self::System {
            content: content.into(),
            name: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::User {
            content: content.into(),
            name: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::Assistant {
            content: content.into(),
            name: None,
            prefix: None,
            reasoning_content: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Thinking {
    Enabled,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    High,
    Max,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopOption {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    Function {
        description: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<serde_json::Value>,
        // default false. beta feature
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ToolChoice {
    General(GeneralToolChoice),
    Named(NamedToolChoice),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneralToolChoice {
    None,
    Auto,
    Required,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NamedToolChoice {
    Function {
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletion {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: u64,
    pub model: Model,
    pub system_fingerprint: String,
    pub object: String,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: FinishReason,
    pub index: u32,
    pub message: ResponseMessage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCall,
    InsufficientSystemResource,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub content: Option<String>,
    pub reasoning_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ResponseToolCall>>,
    pub role: String,
    pub logprobs: Option<LogProbs>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseToolCall {
    Function {
        id: String,
        function: ResponseToolCallFunction,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseToolCallFunction {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogProbs {
    pub content: Option<PredictedTokenLogProb>,
    pub reasoning_content: Option<PredictedTokenLogProb>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictedTokenLogProb {
    pub token: String,
    pub logprob: f32,
    pub bytes: Option<Vec<u8>>,
    pub top_logprobs: Vec<LogProb>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogProb {
    pub token: String,
    pub logprob: f32,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub prompt_cache_hit_tokens: u32,
    pub prompt_cache_miss_tokens: u32,
    pub total_tokens: u32,
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub choices: Vec<ChunkChoice>,
    pub created: u64,
    pub model: Model,
    pub system_fingerprint: String,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkChoice {
    pub delta: ResponseMessage,
    pub logprobs: Option<LogProbs>,
    pub finish_reason: Option<FinishReason>,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkChoiceDelta {
    pub content: Option<String>,
    pub reasoning_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("DEEPSEEK_API_KEY")
            .expect("DEEPSEEK_API_KEY must be set");

        let request = Request::builder(Model::V4Flash)
            .add_message(ChatMessage::user("Hello"))
            .thinking(Thinking::Disabled)
            .max_tokens(50)
            .stream(false)
            .build();

        dbg!("Request: {:?}", serde_json::to_string(&request).unwrap());

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}{}", crate::deepseek::BASE_URL, ENDPOINT))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request)
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success(), "Response status: {}", response.status());

        let completion: ChatCompletion = response
            .json()
            .await
            .expect("Failed to parse response");

        println!("Response: {:?}", completion);
        assert!(!completion.choices.is_empty());
    }
}
