use crate::error::{Result, SmartsError};

/// Represents a message in a conversation
#[derive(Debug, Clone)]
pub enum Message {
    /// System message that sets behavior/context
    System(String),
    /// User message/input
    User(String),
    /// Assistant response (for few-shot examples or conversation history)
    Assistant(String),
}

/// Builder for constructing conversations with proper message formatting
pub struct ConversationBuilder<'a> {
    smarts: &'a crate::models::Smarts,
    messages: Vec<Message>,
}

impl<'a> ConversationBuilder<'a> {
    /// Create a new conversation builder
    pub(crate) fn new(smarts: &'a crate::models::Smarts) -> Self {
        Self {
            smarts,
            messages: Vec::new(),
        }
    }
    
    /// Add a system message to set behavior or context
    pub fn system(mut self, content: &str) -> Self {
        self.messages.push(Message::System(content.to_string()));
        self
    }
    
    /// Add a user message
    pub fn user(mut self, content: &str) -> Self {
        self.messages.push(Message::User(content.to_string()));
        self
    }
    
    /// Add an assistant message (for examples or conversation history)
    pub fn assistant(mut self, content: &str) -> Self {
        self.messages.push(Message::Assistant(content.to_string()));
        self
    }
    
    /// Generate a response from the model
    pub async fn generate(self) -> Result<String> {
        // AI Insight: Validate conversation structure before generation
        // Most chat models expect at least one user message and work best
        // when the conversation ends with a user message (not assistant)
        self.validate_conversation()?;
        
        // Delegate to the Smarts instance for actual generation
        self.smarts.generate_from_conversation(self.messages).await
    }
    
    /// Validate that the conversation structure makes sense
    fn validate_conversation(&self) -> Result<()> {
        if self.messages.is_empty() {
            return Err(SmartsError::ConversationError(
                "Conversation cannot be empty".to_string()
            ));
        }
        
        // Check that we have at least one user message
        let has_user_message = self.messages.iter().any(|msg| matches!(msg, Message::User(_)));
        if !has_user_message {
            return Err(SmartsError::ConversationError(
                "Conversation must contain at least one user message".to_string()
            ));
        }
        
        // AI Insight: Most instruction-tuned models work best when the conversation
        // ends with a user message, as they're trained to respond to user input
        if let Some(last_message) = self.messages.last() {
            if matches!(last_message, Message::Assistant(_)) {
                // This is a warning case, not an error - some use cases might want this
                // for conversation continuation, but it's worth noting
            }
        }
        
        Ok(())
    }
    
    /// Get the messages for inspection (useful for debugging)
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }
}
