use smarts::{Smarts, Result};

#[tokio::test]
async fn test_basic_conversation_api() -> Result<()> {
    // Test that we can create a Smarts instance
    let smarts = Smarts::with_local_model("test-model").await?;
    
    // Test basic conversation building
    let response = smarts
        .conversation()
        .system("You are a helpful assistant")
        .user("Hello, can you help me?")
        .generate()
        .await?;
    
    // Verify we get some response (placeholder for now)
    assert!(!response.is_empty());
    assert!(response.contains("PLACEHOLDER"));
    
    Ok(())
}

#[tokio::test]
async fn test_conversation_validation() -> Result<()> {
    let smarts = Smarts::with_local_model("test-model").await?;
    
    // Test that empty conversation fails
    let result = smarts
        .conversation()
        .generate()
        .await;
    
    assert!(result.is_err());
    
    // Test that conversation without user message fails
    let result = smarts
        .conversation()
        .system("You are helpful")
        .assistant("I'm ready to help")
        .generate()
        .await;
    
    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_model_info() -> Result<()> {
    let smarts = Smarts::with_local_model("TinyLlama-1.1B-Chat").await?;
    
    assert_eq!(smarts.model_info(), "TinyLlama-1.1B-Chat");
    
    Ok(())
}
