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
    pub stop: Option<Stop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    /// [0, 2], default 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// [0, 1], default 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

    pub fn add_message(&mut self, message: impl Into<ChatMessage>) -> &mut Self {
        self.messages.push(message.into());
        self
    }
}

pub struct RequestBuilder {
    model: Model,
    messages: Vec<ChatMessage>,
    thinking: Option<Thinking>,
    reasoning_effort: Option<ReasoningEffort>,
    max_tokens: Option<u32>,
    response_format: Option<ResponseFormat>,
    stop: Option<Stop>,
    stream: Option<bool>,
    stream_options: Option<StreamOptions>,
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
            stream_options: None,
            temperature: None,
            top_p: None,
            tools: Vec::new(),
            tool_choice: None,
            logprobs: None,
            top_logprobs: None,
            user_id: None,
        }
    }

    pub fn add_message(mut self, message: impl Into<ChatMessage>) -> Self {
        self.messages.push(message.into());
        self
    }

    /// Controls the switch between thinking and non-thinking mode.
    /// Default enabled.
    pub fn thinking(mut self, thinking: Thinking) -> Self {
        self.thinking = Some(thinking);
        self
    }

    /// Controls the reasoning effort of the model.
    /// The default effort is high for regular requests; for some complex agent requests (such as Claude Code, OpenCode), effort is automatically set to max. 
    pub fn reasoning_effort(mut self, reasoning_effort: ReasoningEffort) -> Self {
        self.reasoning_effort = Some(reasoning_effort);
        self
    }

    /// The maximum number of tokens that can be generated in the chat completion.
    /// 
    /// The total length of input tokens and generated tokens is limited by the model's context length.
    /// 
    /// For the value range and default value, please refer to the [documentation](https://api-docs.deepseek.com/quick_start/pricing).
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// An object specifying the format that the model must output. Setting to { "type": "json_object" } enables JSON Output, which guarantees the message the model generates is valid JSON.
    ///
    /// Important: When using JSON Output, you must also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly "stuck" request. Also note that the message content may be partially cut off if finish_reason="length", which indicates the generation exceeded max_tokens or the conversation exceeded the max context length.
    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    /// Up to 16 sequences where the API will stop generating further tokens.
    pub fn stop(mut self, stop: Stop) -> Self {
        self.stop = Some(stop);
        self
    }

    /// If set, partial message deltas will be sent. Tokens will be sent as data-only server-sent events (SSE) as they become available, with the stream terminated by a data: [DONE] message.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Options for streaming response. Only set this when you set stream: true.
    pub fn stream_options(mut self, stream_options: StreamOptions) -> Self {
        self.stream_options = Some(stream_options);
        self
    }

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    /// 
    /// We generally recommend altering this or top_p but not both.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or temperature but not both.
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn add_tool(mut self, tool: Tool) -> Self {
        self.tools.push(tool);
        self
    }

    /// A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = tools;
        self
    }

    /// Controls which (if any) tool is called by the model.
    ///
    /// none means the model will not call any tool and instead generates a message.
    ///
    /// auto means the model can pick between generating a message or calling one or more tools.
    ///
    /// required means the model must call one or more tools.
    ///
    /// Specifying a particular tool via {"type": "function", "function": {"name": "my_function"}} forces the model to call that tool.
    ///
    /// none is the default when no tools are present. auto is the default if tools are present.
    pub fn tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    /// Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the content of message.
    pub fn logprobs(mut self, logprobs: bool) -> Self {
        self.logprobs = Some(logprobs);
        self
    }

    /// An integer between 0 and 20 specifying the number of most likely tokens to return at each token position, each with an associated log probability. logprobs must be set to true if this parameter is used.
    pub fn top_logprobs(mut self, top_logprobs: u32) -> Self {
        self.top_logprobs = Some(top_logprobs);
        self
    }

    /// A custom user_id. Allowed character set is [a-zA-Z0-9\-_], with a maximum length of 512. Do not include user privacy information in the user_id.
    ///
    /// user_id can be used to distinguish user identities on your side to help us with content safety review.
    /// 
    /// user_id can be used for KVCache isolation for privacy management.
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
            stream_options: self.stream_options,
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
        /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    User {
        content: String,
        /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Assistant(ResponseMessage),
    Tool {
        content: String,
        /// Tool call that this message is responding to.
        tool_call_id: String,
    },
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        ChatMessage::System {
            content: content.into(),
            name: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        ChatMessage::User {
            content: content.into(),
            name: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        ChatMessage::Assistant(ResponseMessage {
            content: Some(content.into()),
            reasoning_content: None,
            tool_calls: Vec::new(),
            role: "assistant".to_string(),
            logprobs: None,
        })
    }
}

impl Into<ChatMessage> for String {
    fn into(self) -> ChatMessage {
        ChatMessage::User {
            content: self,
            name: None,
        }
    }
}

impl Into<ChatMessage> for &str {
    fn into(self) -> ChatMessage {
        ChatMessage::User {
            content: self.to_string(),
            name: None,
        }
    }
}

impl Into<ChatMessage> for ResponseMessage {
    fn into(mut self) -> ChatMessage {
        // remove unnecessary fields for the next turn of conversation
        self.logprobs = None;
        ChatMessage::Assistant(self)
    }
}

impl Into<ChatMessage> for &ResponseMessage {
    fn into(self) -> ChatMessage {
        let message = self.clone();
        message.into()
    }
}

impl Into<ChatMessage> for Choice {
    fn into(self) -> ChatMessage {
        self.message.into()
    }
}

impl Into<ChatMessage> for &Choice {
    fn into(self) -> ChatMessage {
        self.message.clone().into()
    }
}

impl Into<ChatMessage> for ChatCompletion {
    fn into(self) -> ChatMessage {
        self.assert_one_choice().into()
    }
}

impl Into<ChatMessage> for &ChatCompletion {
    fn into(self) -> ChatMessage {
        self.assert_one_choice().into()
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
pub enum Stop {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamOptions {
    include_usage: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    Function {
        function: ToolFunctionDefinition,
    }
}

impl Tool {
    /// The name of the function to be called must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub fn function_builder(name: impl Into<String>) -> ToolFunctionBuilder {
        ToolFunctionBuilder {
            name: name.into(),
            description: None,
            parameters: None,
            strict: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolFunctionDefinition {
    /// A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the function to be called.
    /// Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: String,
    /// The parameters the functions accepts, described as a JSON Schema object.
    /// See the [Tool Calls Guide](https://api-docs.deepseek.com/guides/tool_calls) for examples,
    /// and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format.
    ///
    /// Omitting parameters defines a function with an empty parameter list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    /// If set to true, the API will use strict-mode for the tool calls to ensure the output always complies with the function's JSON schema.
    /// This is a Beta feature, for more details please refer to [Tool Calls Guide](https://api-docs.deepseek.com/guides/tool_calls)
    /// 
    /// Default value: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

pub struct ToolFunctionBuilder {
    name: String,
    description: Option<String>,
    parameters: Option<serde_json::Value>,
    strict: Option<bool>,
}

impl ToolFunctionBuilder {
    /// A description of what the function does, used by the model to choose when and how to call the function.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The parameters the functions accepts, described as a JSON Schema object.
    /// See the [Tool Calls Guide](https://api-docs.deepseek.com/guides/tool_calls) for examples,
    /// and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format.
    ///
    /// Omitting parameters defines a function with an empty parameter list.
    pub fn parameters(mut self, parameters: serde_json::Value) -> Self {
        self.parameters = Some(parameters);
        self
    }

    /// If set to true, the API will use strict-mode for the tool calls to ensure the output always complies with the function's JSON schema.
    /// This is a Beta feature, for more details please refer to [Tool Calls Guide](https://api-docs.deepseek.com/guides/tool_calls)
    /// 
    /// Default value: false
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = Some(strict);
        self
    }

    pub fn build(self) -> Tool {
        Tool::Function {
            function: ToolFunctionDefinition {
                name: self.name,
                description: self.description,
                parameters: self.parameters,
                strict: self.strict,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ToolChoice {
    General(GeneralToolChoice),
    Named(NamedToolChoice),
}

impl ToolChoice {
    pub fn named_function(function_name: impl Into<String>) -> Self {
        ToolChoice::Named(NamedToolChoice::Function {
            function: NamedToolChoiceFunction {
                name: function_name.into(),
            },
        })
    }
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
        function: NamedToolChoiceFunction,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedToolChoiceFunction {
    /// The name of the function to call.
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletion {
    /// A unique identifier for the chat completion.
    pub id: String,
    /// A list of chat completion choices.
    pub choices: Vec<Choice>,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: u64,
    /// The model used for the chat completion.
    pub model: Model,
    /// This fingerprint represents the backend configuration that the model runs with.
    pub system_fingerprint: String,
    /// The object type, which is always chat.completion.
    pub object: String,
    /// Usage statistics for the completion request.
    pub usage: Usage,
}

impl ChatCompletion {
    pub fn assert_one_choice(&self) -> &Choice {
        assert_eq!(self.choices.len(), 1, "Expected exactly one choice, got {}", self.choices.len());
        &self.choices[0]
    }

    pub fn merge_from_chunks(chunks: Vec<ChatCompletionChunk>) -> Self {
        assert!(!chunks.is_empty(), "No chunks to merge");
        let first_chunk = &chunks[0];
        let id = first_chunk.id.clone();
        let created = first_chunk.created;
        let model = first_chunk.model;
        let system_fingerprint = first_chunk.system_fingerprint.clone();

        let mut finish_reason = None;
        let mut content: Option<String> = None;
        let mut reasoning_content: Option<String> = None;
        let mut tool_calls: Vec<ToolCallDelta> = Vec::new();
        let mut usage = None;
        
        for mut chunk in chunks {
            if chunk.choices.is_empty() {
                continue;
            }
            let chunk_choice = chunk.choices.remove(0);
            if let Some(chunk_finish_reason) = chunk_choice.finish_reason {
                finish_reason = Some(chunk_finish_reason);
            }

            let chunk_choice_delta = chunk_choice.delta;
            if let Some(chunk_content) = chunk_choice_delta.content {
                if let Some(ref mut content) = content {
                    content.push_str(&chunk_content);
                } else {
                    content = Some(chunk_content);
                }
            }
            if let Some(chunk_reasoning_content) = chunk_choice_delta.reasoning_content {
                if let Some(ref mut reasoning) = reasoning_content {
                    reasoning.push_str(&chunk_reasoning_content);
                } else {
                    reasoning_content = Some(chunk_reasoning_content);
                }
            }

            for tool_call_delta in chunk_choice_delta.tool_calls {
                let index = tool_call_delta.index as usize;
                if index < tool_calls.len() {
                    // update existing tool call
                    let existing_tool_call = &mut tool_calls[index];
                    if let Some(id) = &tool_call_delta.id {
                        existing_tool_call.id = Some(id.clone());
                    }
                    if let Some(name) = &tool_call_delta.function.name {
                        existing_tool_call.function.name = Some(name.clone());
                    }
                    if let Some(arguments) = &tool_call_delta.function.arguments {
                        if let Some(existing_arguments) = &mut existing_tool_call.function.arguments {
                            existing_arguments.push_str(arguments);
                        } else {
                            existing_tool_call.function.arguments = Some(arguments.clone());
                        }
                    }
                } else {
                    for i in tool_calls.len()..index {
                        // fill the gap with empty tool calls if there are missing indices
                        tool_calls.push(ToolCallDelta {
                            id: None,
                            function: ToolCallFunctionDelta {
                                name: None,
                                arguments: None,
                            },
                            index: i as u32,
                        });
                    }
                    // add new tool call
                    tool_calls.push(tool_call_delta);
                }
            }

            if let Some(chunk_usage) = chunk.usage {
                usage = Some(chunk_usage);
            }
        }

        let tool_calls = tool_calls.into_iter().map(|delta| ToolCall::Function {
            id: delta.id.expect("tool call id not found"),
            function: ToolCallFunction {
                name: delta.function.name.expect("tool call function name not found"),
                arguments: serde_json::from_str(&delta.function.arguments.expect("tool call arguments not found")).unwrap(),
            },
        }).collect();

        let choices = vec![Choice {
            finish_reason: finish_reason.unwrap_or(FinishReason::Other),
            index: 0,
            message: ResponseMessage {
                content,
                reasoning_content,
                tool_calls,
                role: "assistant".to_string(),
                logprobs: None,
            },
        }];

        ChatCompletion {
            id,
            choices,
            created,
            model,
            system_fingerprint,
            object: "chat.completion".to_string(),
            usage: usage.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    /// The reason the model stopped generating tokens.
    /// This will be stop if the model hit a natural stop point or a provided stop sequence,
    /// length if the maximum number of tokens specified in the request was reached,
    /// content_filter if content was omitted due to a flag from our content filters,
    /// tool_calls if the model called a tool,
    /// or insufficient_system_resource if the request is interrupted due to insufficient resource of the inference system.
    pub finish_reason: FinishReason,
    /// The index of the choice in the list of choices.
    pub index: u32,
    /// A chat completion message generated by the model.
    pub message: ResponseMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
    InsufficientSystemResource,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMessage {
    /// The contents of the message.
    pub content: Option<String>,
    /// For thinking mode only.
    /// The reasoning contents of the assistant message, before the final answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    /// The tool calls generated by the model.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
    /// Possible values: [assistant]
    /// The role of the author of this message.
    #[serde(skip_serializing)]
    pub role: String,
    /// Log probability information for the choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolCall {
    Function {
        /// The ID of the tool call.
        id: String,
        /// The function that the model called.
        function: ToolCallFunction,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallFunction {
    /// The name of the function to call.
    pub name: String,
    /// The arguments to call the function with, as generated by the model in JSON format.
    /// Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema.
    /// Validate the arguments in your code before calling your function.
    #[serde(deserialize_with = "from_json_string", serialize_with = "to_json_string")]
    pub arguments: serde_json::Value,
}

fn from_json_string<'de, D>(deserializer: D) -> Result<serde_json::Value, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // 先解析为 String
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    // 再将 String 解析为 Value
    serde_json::from_str(&s).map_err(serde::de::Error::custom)
}

fn to_json_string<S>(value: &serde_json::Value, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = serde_json::to_string(value).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogProbs {
    /// A list of message content tokens with log probability information.
    pub content: Option<PredictedTokenLogProb>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of message content tokens with log probability information.
    pub reasoning_content: Option<PredictedTokenLogProb>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PredictedTokenLogProb {
    /// The token.
    pub token: String,
    /// The log probability of this token, if it is within the top 20 most likely tokens.
    /// Otherwise, the value -9999.0 is used to signify that the token is very unlikely.
    pub logprob: f32,
    /// A list of integers representing the UTF-8 bytes representation of the token.
    /// Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation.
    /// Can be null if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,
    /// List of the most likely tokens and their log probability, at this token position.
    /// In rare cases, there may be fewer than the number of requested top_logprobs returned.
    pub top_logprobs: Vec<LogProb>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogProb {
    /// The token.
    pub token: String,
    /// The log probability of this token, if it is within the top 20 most likely tokens.
    /// Otherwise, the value -9999.0 is used to signify that the token is very unlikely.
    pub logprob: f32,
    /// A list of integers representing the UTF-8 bytes representation of the token.
    /// Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation.
    /// Can be null if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Usage {
    /// Number of tokens in the generated completion.
    pub completion_tokens: u32,
    /// Number of tokens in the prompt.
    /// It equals prompt_cache_hit_tokens + prompt_cache_miss_tokens.
    pub prompt_tokens: u32,
    /// Number of tokens in the prompt that hits the context cache.
    pub prompt_cache_hit_tokens: u32,
    /// Number of tokens in the prompt that misses the context cache.
    pub prompt_cache_miss_tokens: u32,
    /// Total number of tokens used in the request (prompt + completion).
    pub total_tokens: u32,
    /// Breakdown of tokens used in a completion.
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CompletionTokensDetails {
    /// Tokens generated by the model for reasoning.
    pub reasoning_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub choices: Vec<ChunkChoice>,
    pub created: u64,
    pub model: Model,
    pub system_fingerprint: String,
    pub object: String,
    pub usage: Option<Usage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkChoice {
    pub delta: ChunkChoiceDelta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
    pub finish_reason: Option<FinishReason>,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkChoiceDelta {
    pub content: Option<String>,
    pub reasoning_content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCallDelta>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ToolCallDelta {
    pub id: Option<String>,
    pub function: ToolCallFunctionDelta,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallFunctionDelta {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::{self, Write}, sync::Once};

    use reqwest_sse::EventSource;
    use tokio_stream::StreamExt;

    static INIT: Once = Once::new();

    fn api_key() -> String {
        INIT.call_once(|| {
            dotenv::dotenv().ok();
        });
        std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set")
    }

    async fn send_request(client: &reqwest::Client, request: &Request) -> ChatCompletion {
        let api_key = api_key();
        let response = client
            .post(format!("{}{}", crate::deepseek::BASE_URL, ENDPOINT))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(request)
            .send()
            .await
            .expect("Failed to send request");
        assert!(response.status().is_success(), "Response status: {}", response.status());
        // let raw_response = response.text().await.expect("Failed to read response text");
        // dbg!("Raw response: {}", &raw_response);
        // serde_json::from_str(&raw_response).expect("Failed to parse response")
        response.json().await.expect("Failed to parse response")
    }

    async fn send_streaming_request(client: &reqwest::Client, request: &Request) -> Vec<ChatCompletionChunk> {
        let api_key = api_key();
        let response = client
            .post(format!("{}{}", crate::deepseek::BASE_URL, ENDPOINT))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(request)
            .send()
            .await
            .expect("Failed to send request");
        assert!(response.status().is_success(), "Response status: {}", response.status());

        let mut events = response.events().await.unwrap();

        let mut chunks = Vec::new();
        while let Some(Ok(event)) = events.next().await {
            if event.data == "[DONE]" {
                println!("Stream finished");
                break;
            }
            let json_value = serde_json::from_str::<serde_json::Value>(&event.data).unwrap();
            println!("Received chunk data: \n{}", serde_json::to_string_pretty(&json_value).unwrap());
            let chunk: ChatCompletionChunk = serde_json::from_str(&event.data).expect("Failed to parse chunk");
            if !chunk.choices.is_empty()
                && let Some(ref content) = chunk.choices[0].delta.content
            {
                io::stdout().write_all(content.as_bytes()).unwrap();
                io::stdout().flush().unwrap();
            }
            if let Some(usage) = &chunk.usage {
                println!("\nusage {:?}", usage);
            }
            chunks.push(chunk);
        }

        chunks
    }

    fn merge_chunks(chunks: Vec<ChatCompletionChunk>) -> String {
        chunks.into_iter()
            .filter(|chunk| !chunk.choices.is_empty())
            .map(|chunk| chunk.choices[0].delta.content.clone().unwrap_or_default())
            .collect::<Vec<_>>()
            .join("")
    }

    #[tokio::test]
    async fn test_hello() {
        let client = reqwest::Client::new();

        let request = Request::builder(Model::V4Flash)
            .add_message("Hello")
            .thinking(Thinking::Disabled)
            .max_tokens(50)
            .stream(false)
            .build();

        let completion = send_request(&client, &request).await;

        println!("Response: {:?}", completion);
        assert!(!completion.assert_one_choice().message.content.as_ref().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_multi_turn() {
        let client = reqwest::Client::new();

        let mut request = Request::builder(Model::V4Flash)
            .add_message("Hello")
            .add_message(ChatMessage::assistant("Hi! How can I assist you today?"))
            .add_message("Can you tell me a joke?")
            .thinking(Thinking::Disabled)
            .max_tokens(500)
            .stream(false)
            .build();

        let completion = send_request(&client, &request).await;

        println!("Response: {:?}", completion);
        assert!(!completion.assert_one_choice().message.content.as_ref().unwrap().is_empty());

        request.add_message(completion);

        let completion = send_request(&client, &request).await;

        println!("Response: {:?}", completion);
        assert!(!completion.assert_one_choice().message.content.as_ref().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_streaming() {
        let client = reqwest::Client::new();

        let data = fs::read_to_string("test_data/yuanjun.txt").expect("Failed to read test data");
        let mut request = Request::builder(Model::V4Flash)
            .add_message(data)
            .add_message("Please summarize the above content.")
            .thinking(Thinking::Enabled)
            .max_tokens(5000)
            .stream(true)
            .stream_options(StreamOptions { include_usage: true })
            .build();

        let completion_chunks = send_streaming_request(&client, &request).await;
        assert!(!completion_chunks.is_empty());
        println!("chunk count: {}", completion_chunks.len());


        let response_message = merge_chunks(completion_chunks);
        request.add_message(ChatMessage::assistant(response_message));
        request.add_message("请将这段内容翻译成英文。");

        let completion_chunks = send_streaming_request(&client, &request).await;
        assert!(!completion_chunks.is_empty());
        println!("chunk count: {}", completion_chunks.len());
    }

    #[tokio::test]
    async fn test_tools() {
        let client = reqwest::Client::new();

        let mut request = Request::builder(Model::V4Flash)
            .add_message(ChatMessage::user("What is the weather like in WuHan?"))
            .add_tool(Tool::function_builder("get_current_weather")
                .description("Get the current weather in a given location")
                .parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        }
                    },
                    "required": ["location"]
                }))
                .build())
            .add_tool(Tool::function_builder("get_family_members")
                .description("Get family members of a person")
                .parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "person_name": {
                            "type": "string",
                            "description": "The name of the person, e.g. John Doe"
                        }
                    },
                    "required": ["person_name"]
                }))
                .build())
            .tool_choice(ToolChoice::General(GeneralToolChoice::Auto))
            .build();

        println!("first request: \n{}", serde_json::to_string_pretty(&request).unwrap());
        let completion = send_request(&client, &request).await;

        println!("Response: {:?}", completion);
        assert_eq!(completion.assert_one_choice().message.tool_calls.len(), 1);

        request.add_message(&completion);
        let tool_call = &completion.assert_one_choice().message.tool_calls[0];
        match tool_call {
            ToolCall::Function { id, function } => {
                assert_eq!(function.name, "get_current_weather");
                let location = function.arguments.get("location").unwrap().as_str().unwrap();
                println!("Tool call arguments: location={}", location);
                request.add_message(ChatMessage::Tool {
                    content: "Rain".to_string(),
                    tool_call_id: id.clone(),
                });
            }
        }

        println!("Request after tool call response: \n{}", serde_json::to_string_pretty(&request).unwrap());
        let completion = send_request(&client, &request).await;
        println!("Response: {:?}", completion);

        request.add_message(&completion);
        request.add_message("What are the family members of John Doe?");
        let completion = send_request(&client, &request).await;
        println!("Response: {:?}", completion);
        assert_eq!(completion.assert_one_choice().message.tool_calls.len(), 1);

        request.add_message(&completion);
        let tool_call = &completion.assert_one_choice().message.tool_calls[0];
        match tool_call {
            ToolCall::Function { id, function } => {
                assert_eq!(function.name, "get_family_members");
                let person_name = function.arguments.get("person_name").unwrap().as_str().unwrap();
                assert_eq!(person_name, "John Doe");
                request.add_message(ChatMessage::Tool {
                    content: "Jane Doe, Jack Doe".to_string(),
                    tool_call_id: id.clone(),
                });
            }
        }
        let completion = send_request(&client, &request).await;
        println!("Response: {:?}", completion);
    }

    #[tokio::test]
    async fn test_streaming_tools() {
        let client = reqwest::Client::new();

        let mut request = Request::builder(Model::V4Flash)
            .add_message(ChatMessage::user("What is the weather like in WuHan?"))
            .add_tool(Tool::function_builder("get_current_weather")
                .description("Get the current weather in a given location")
                .parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        }
                    },
                    "required": ["location"]
                }))
                .build())
            .add_tool(Tool::function_builder("get_family_members")
                .description("Get family members of a person")
                .parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "person_name": {
                            "type": "string",
                            "description": "The name of the person, e.g. John Doe"
                        }
                    },
                    "required": ["person_name"]
                }))
                .build())
            .tool_choice(ToolChoice::General(GeneralToolChoice::Auto))
            .stream(true)
            .build();

        println!("first request: \n{}", serde_json::to_string_pretty(&request).unwrap());
        let chunks = send_streaming_request(&client, &request).await;
        let completion = ChatCompletion::merge_from_chunks(chunks.clone());

        println!("Response: {:?}", completion);
        request.add_message(&completion);

        let tool_call = &completion.assert_one_choice().message.tool_calls[0];
        match tool_call {
            ToolCall::Function { id, function } => {
                assert_eq!(function.name, "get_current_weather");
                let location = function.arguments.get("location").unwrap().as_str().unwrap();
                println!("Tool call arguments: location={}", location);
                request.add_message(ChatMessage::Tool {
                    content: "Rain".to_string(),
                    tool_call_id: id.clone(),
                });
            }
        }
        let chunks = send_streaming_request(&client, &request).await;
        let completion = ChatCompletion::merge_from_chunks(chunks.clone());
        println!("Response: {:?}", completion);
    }
}
