//! # Fluix AI Components
//! 
//! A comprehensive collection of AI-focused UI components for building modern AI applications.
//! 
//! ## Modules
//!
//! - [`prompt`] - AI prompt input component (PromptInput)
//! - [`message`] - Message component for displaying AI conversation messages (MessageBubble)
//! - [`model_selector`] - AI model selection component (ModelSelector)
//! 
//! ## Quick Start
//! 
//! ```rust
//! use fluix::ai::*;
//! 
//! // Create a simple chat interface
//! ChatContainer::new(cx)
//!     .with_prompt_input(
//!         PromptInput::new(cx)
//!             .placeholder("Ask me anything...")
//!             .enable_attachments(true)
//!     )
//! ```

pub mod prompt;
pub mod message;
pub mod model_selector;

// Re-export commonly used types
pub use prompt::*;
pub use message::*;
pub use model_selector::*;

// Common types and traits used across AI components
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for messages
pub type MessageId = Uuid;

/// Represents the role of a message sender in AI conversations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    /// Human user message
    User,
    /// AI assistant message
    Assistant,
    /// System message (instructions, notifications)
    System,
    /// Tool/function call result
    Tool,
}

/// Content types that can be displayed in AI messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    /// Plain text content
    Text(String),
    /// Code block with syntax highlighting
    Code { 
        language: String, 
        code: String 
    },
    /// Terminal command and output
    Terminal { 
        command: String, 
        output: String 
    },
    /// Image with metadata
    Image { 
        url: String, 
        alt: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    /// File attachment
    File { 
        path: String, 
        name: String,
        size: u64,
        mime_type: Option<String>,
    },
    /// AI thinking process (internal reasoning)
    Thinking(String),
    /// Error message
    Error(String),
    /// Tool/function call
    Tool { 
        name: String, 
        input: String, 
        output: String 
    },
}

/// A message in an AI conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique identifier
    pub id: MessageId,
    /// Message sender role
    pub role: MessageRole,
    /// Message content (can contain multiple content blocks)
    pub content: Vec<MessageContent>,
    /// When the message was created
    pub timestamp: DateTime<Utc>,
    /// Whether the message is currently being streamed
    pub is_streaming: bool,
    /// Optional metadata
    pub metadata: Option<serde_json::Value>,
}

impl Message {
    /// Create a new user message with text content
    pub fn new_user(text: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::User,
            content: vec![MessageContent::Text(text.into())],
            timestamp: Utc::now(),
            is_streaming: false,
            metadata: None,
        }
    }

    /// Create a new assistant message with content
    pub fn new_assistant(content: Vec<MessageContent>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::Assistant,
            content,
            timestamp: Utc::now(),
            is_streaming: false,
            metadata: None,
        }
    }

    /// Create a new system message
    pub fn new_system(text: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::System,
            content: vec![MessageContent::Text(text.into())],
            timestamp: Utc::now(),
            is_streaming: false,
            metadata: None,
        }
    }

    /// Get the primary text content of the message
    pub fn get_text(&self) -> String {
        self.content
            .iter()
            .filter_map(|content| match content {
                MessageContent::Text(text) => Some(text.clone()),
                MessageContent::Thinking(text) => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Check if the message contains code
    pub fn has_code(&self) -> bool {
        self.content.iter().any(|content| matches!(content, MessageContent::Code { .. }))
    }

    /// Check if the message contains attachments
    pub fn has_attachments(&self) -> bool {
        self.content.iter().any(|content| {
            matches!(content, MessageContent::Image { .. } | MessageContent::File { .. })
        })
    }
}

/// Information about an AI model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Model description
    pub description: Option<String>,
    /// Provider/company
    pub provider: String,
    /// Context window size
    pub context_length: Option<u32>,
    /// Model capabilities
    pub capabilities: Vec<ModelCapability>,
    /// Pricing information
    pub pricing: Option<PricingInfo>,
}

impl ModelInfo {
    /// Get default models from llm-link crate
    /// This function loads all available models from llm-link providers dynamically
    pub fn default_models_from_llm_link() -> Vec<Self> {
        use llm_link::models::ModelsConfig;
        
        let mut models = Vec::new();
        
        // Load models configuration from llm-link
        let models_config = ModelsConfig::load_with_fallback();
        
        // Get all providers dynamically from llm-link
        let provider_ids = models_config.get_all_providers();
        
        for provider_id in provider_ids {
            // Get models for each provider from ModelsConfig
            let provider_models = models_config.get_models_for_provider(&provider_id);
            
            // Format provider name for display (capitalize first letter)
            let provider_name = Self::format_provider_name(&provider_id);
            
            for model_info in provider_models {
                // Convert llm-link ModelInfo to our ModelInfo
                // Note: llm-link's ModelInfo has: id (String), name (String), description (String)
                let model = ModelInfo {
                    id: model_info.id.clone(),
                    name: model_info.name.clone(),
                    description: Some(model_info.description.clone()), // Wrap String in Some for Option<String>
                    provider: provider_name.clone(),
                    context_length: None, // llm-link doesn't provide context_length in ModelInfo
                    capabilities: Self::infer_capabilities_from_provider(&provider_id),
                    pricing: None, // llm-link doesn't provide pricing info
                };
                
                models.push(model);
            }
        }
        
        models
    }
    
    /// Format provider ID to display name
    /// Converts lowercase provider IDs (e.g., "openai") to formatted names (e.g., "OpenAI")
    fn format_provider_name(provider_id: &str) -> String {
        // Handle special cases for known providers
        match provider_id {
            "openai" => "OpenAI".to_string(),
            "anthropic" => "Anthropic".to_string(),
            "zhipu" => "Zhipu".to_string(),
            "aliyun" => "Aliyun".to_string(),
            "volcengine" => "Volcengine".to_string(),
            "tencent" => "Tencent".to_string(),
            "longcat" => "Longcat".to_string(),
            "moonshot" => "Moonshot".to_string(),
            "ollama" => "Ollama".to_string(),
            _ => {
                // Default: capitalize first letter
                let mut chars = provider_id.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            }
        }
    }
    
    /// Infer capabilities from provider name
    fn infer_capabilities_from_provider(provider: &str) -> Vec<ModelCapability> {
        // Default capabilities - most modern LLMs support these
        let mut capabilities = vec![
            ModelCapability::TextGeneration,
            ModelCapability::CodeGeneration,
        ];
        
        // Add provider-specific capabilities
        match provider {
            "openai" | "anthropic" | "zhipu" | "moonshot" => {
                capabilities.push(ModelCapability::FunctionCalling);
            }
            _ => {}
        }
        
        capabilities
    }
}

/// Model capabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelCapability {
    /// Text generation and completion
    TextGeneration,
    /// Code generation and completion
    CodeGeneration,
    /// Image generation
    ImageGeneration,
    /// Image analysis and understanding
    ImageAnalysis,
    /// Function/tool calling
    FunctionCalling,
    /// Document analysis
    DocumentAnalysis,
    /// Web search
    WebSearch,
}

/// Pricing information for a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingInfo {
    /// Input token price (per 1K tokens)
    pub input_price: f64,
    /// Output token price (per 1K tokens)
    pub output_price: f64,
    /// Currency code (e.g., "USD")
    pub currency: String,
}

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Provider identifier
    pub id: String,
    /// Provider name
    pub name: String,
    /// Available models
    pub models: Vec<ModelInfo>,
    /// API endpoint
    pub endpoint: Option<String>,
    /// Whether authentication is required
    pub requires_auth: bool,
}

impl ProviderInfo {
    /// Get all providers from llm-link crate with their models
    /// This function loads all available providers and models dynamically
    pub fn all_from_llm_link() -> Vec<Self> {
        use llm_link::models::ModelsConfig;
        
        let mut providers = Vec::new();
        
        // Load models configuration from llm-link
        let models_config = ModelsConfig::load_with_fallback();
        
        // Get all providers dynamically from llm-link
        let provider_ids = models_config.get_all_providers();
        
        for provider_id in provider_ids {
            // Get models for this provider
            let provider_models = models_config.get_models_for_provider(&provider_id);
            
            // Convert llm-link models to our ModelInfo
            let models: Vec<ModelInfo> = provider_models
                .into_iter()
                .map(|model_info| {
                    let provider_name = ModelInfo::format_provider_name(&provider_id);
                    ModelInfo {
                        id: model_info.id.clone(),
                        name: model_info.name.clone(),
                        description: Some(model_info.description.clone()),
                        provider: provider_name.clone(),
                        context_length: None,
                        capabilities: ModelInfo::infer_capabilities_from_provider(&provider_id),
                        pricing: None,
                    }
                })
                .collect();
            
            // Format provider name for display
            let provider_name = ModelInfo::format_provider_name(&provider_id);
            
            let provider_info = ProviderInfo {
                id: provider_id.clone(),
                name: provider_name,
                models,
                endpoint: None, // llm-link doesn't provide endpoint info in ModelsConfig
                requires_auth: true, // Most providers require auth
            };
            
            providers.push(provider_info);
        }
        
        providers
    }
}

/// Attachment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Attachment identifier
    pub id: String,
    /// File name
    pub name: String,
    /// File path or URL
    pub path: String,
    /// File size in bytes
    pub size: u64,
    /// MIME type
    pub mime_type: String,
    /// Attachment type
    pub attachment_type: AttachmentType,
}

/// Types of attachments
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentType {
    /// Image file
    Image,
    /// Document file
    Document,
    /// Code file
    Code,
    /// Audio file
    Audio,
    /// Video file
    Video,
    /// Other file type
    Other,
}

impl Attachment {
    /// Create a new attachment
    pub fn new(
        name: impl Into<String>,
        path: impl Into<String>,
        size: u64,
        mime_type: impl Into<String>,
    ) -> Self {
        let mime_type = mime_type.into();
        let attachment_type = AttachmentType::from_mime_type(&mime_type);
        
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            path: path.into(),
            size,
            mime_type,
            attachment_type,
        }
    }
}

impl AttachmentType {
    /// Determine attachment type from MIME type
    pub fn from_mime_type(mime_type: &str) -> Self {
        match mime_type.split('/').next() {
            Some("image") => Self::Image,
            Some("audio") => Self::Audio,
            Some("video") => Self::Video,
            Some("text") => Self::Code,
            Some("application") => {
                if mime_type.contains("pdf") || mime_type.contains("document") {
                    Self::Document
                } else {
                    Self::Other
                }
            }
            _ => Self::Other,
        }
    }
}
