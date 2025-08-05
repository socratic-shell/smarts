use std::fmt;

/// Result type alias for Smarts operations
pub type Result<T> = std::result::Result<T, SmartsError>;

/// Error types for Smarts operations
#[derive(Debug)]
pub enum SmartsError {
    /// Model loading or initialization failed
    ModelError(String),
    
    /// Model inference failed
    InferenceError(String),
    
    /// Invalid conversation structure
    ConversationError(String),
    
    /// I/O errors (file operations, network, etc.)
    IoError(std::io::Error),
    
    /// Configuration or setup errors
    ConfigError(String),
}

impl fmt::Display for SmartsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartsError::ModelError(msg) => write!(f, "Model error: {}", msg),
            SmartsError::InferenceError(msg) => write!(f, "Inference error: {}", msg),
            SmartsError::ConversationError(msg) => write!(f, "Conversation error: {}", msg),
            SmartsError::IoError(err) => write!(f, "I/O error: {}", err),
            SmartsError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for SmartsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SmartsError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SmartsError {
    fn from(err: std::io::Error) -> Self {
        SmartsError::IoError(err)
    }
}
