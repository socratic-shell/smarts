//! # Smarts
//! 
//! A library for managing locally installed LLM models and embeddings.
//! Provides a unified interface for MCP servers to access AI capabilities.

mod conversation;
mod models;
mod error;

pub use conversation::{ConversationBuilder, Message};
pub use models::Smarts;
pub use error::{SmartsError, Result};

// Re-export commonly used types
pub use tokio;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_api_compiles() {
        // Just verify the basic API structure compiles
        // Real tests will be added as we implement functionality
    }
}
