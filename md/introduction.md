# Smarts Library

Smarts is a Rust library for managing locally installed LLM models and embeddings. It provides a unified interface for MCP servers to access AI capabilities without requiring external services.

## Key Features

- **Chat-style API**: Natural conversation builder with system/user/assistant messages
- **Local Models**: Self-contained execution using small, quantized instruction-tuned models
- **Stateless Design**: MCP servers manage conversation state, library handles inference
- **Async Support**: Built with tokio for non-blocking operations
- **Extensible**: Designed to support multiple backends (local models, MCP sampling, remote APIs)

## Target Use Cases

- **Context Summarization**: Track conversation context across chat sessions
- **Content Classification**: Categorize user requests and messages
- **Text Processing**: Basic NLP tasks like summarization and analysis
- **MCP Server Enhancement**: Add AI capabilities to existing MCP servers

## Design Philosophy

The library prioritizes:

1. **Simplicity**: Clean, intuitive API that's easy to integrate
2. **Local-first**: No external dependencies or network requirements
3. **Resource Efficiency**: Small models that run well on typical hardware
4. **Flexibility**: Extensible architecture for future enhancements

## Current Status

**MVP Phase**: Basic conversation API with placeholder model execution. The core API structure is complete and tested, with actual model inference coming in the next iteration.

## Quick Example

```rust
use smarts::{Smarts, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let smarts = Smarts::with_local_model("TinyLlama-1.1B-Chat").await?;
    
    let response = smarts
        .conversation()
        .system("You provide concise summaries")
        .user("Summarize: User is debugging network issues")
        .generate()
        .await?;
    
    println!("Summary: {}", response);
    Ok(())
}
```
