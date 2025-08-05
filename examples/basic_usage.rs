use smarts::{Smarts, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Smarts Library - Basic Usage Example");
    println!("====================================");
    
    // Create a Smarts instance with a local model
    let smarts = Smarts::with_local_model("TinyLlama-1.1B-Chat").await?;
    println!("Loaded model: {}", smarts.model_info());
    
    // Example 1: Simple conversation
    println!("\n--- Example 1: Simple Conversation ---");
    let response = smarts
        .conversation()
        .system("You are a helpful assistant that provides concise answers.")
        .user("What is the capital of France?")
        .generate()
        .await?;
    
    println!("Response: {}", response);
    
    // Example 2: Context summarization (typical MCP server use case)
    println!("\n--- Example 2: Context Summarization ---");
    let current_summary = "User is debugging network connectivity issues";
    let new_message = "User reports that ping works but HTTP requests are timing out";
    
    let updated_summary = smarts
        .conversation()
        .system("You provide concise context summaries for ongoing conversations.")
        .user(&format!(
            "Update this summary with new information:\nCurrent: {}\nNew message: {}",
            current_summary, new_message
        ))
        .generate()
        .await?;
    
    println!("Updated summary: {}", updated_summary);
    
    // Example 3: Few-shot example with assistant message
    println!("\n--- Example 3: Few-shot Example ---");
    let response = smarts
        .conversation()
        .system("You categorize user requests into: question, task, or problem")
        .user("How do I install Python?")
        .assistant("Category: question")
        .user("My computer won't start")
        .generate()
        .await?;
    
    println!("Categorization: {}", response);
    
    Ok(())
}
