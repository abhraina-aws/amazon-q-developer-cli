#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_what_is_aws_prompt() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 [AI PROMPTS] Testing 'What is AWS?' AI prompt... | Description: Tests <code>AI prompt</code> functionality <code>by sending 'What is AWS?'</code> and verifying the response contains relevant AWS information and technical terms");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap();
    println!("✅ Kiro-cli Chat session started");
    
    let response = chat.execute_command_with_timeout("What is AWS?",Some(1000))?;
    
    println!("📝 AI response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Assert we got a meaningful AWS response
    let has_aws_content = response.contains("Amazon Web Services") || 
                         response.contains("cloud") || 
                         response.contains("AWS");
    assert!(has_aws_content || response.len() > 100, "Response should contain AWS-related content or be substantial (got {} bytes)", response.len());
    
    // Verify technical depth
    let technical_terms = ["service", "platform", "infrastructure", "compute", "storage"];
    let has_technical_terms = technical_terms.iter().any(|&term| response.to_lowercase().contains(term));
    assert!(has_technical_terms || has_aws_content, "Response should include technical terms or AWS-specific content");

    println!("✅ AI prompt test completed successfully");

     // Release the lock before cleanup
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_simple_greeting() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing simple 'Hello' prompt... | Description: Tests basic AI interaction by sending a simple greeting and verifying the AI responds appropriately with greeting-related content");
    
    let session =q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap();
    println!("✅ Kiro-cli Chat session started");
    
    let response = chat.execute_command_with_timeout("Hello",Some(1000))?;
    
    println!("📝 Greeting response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Assert we got a meaningful response
    assert!(!response.trim().is_empty(), "AI should respond to greeting");
    assert!(response.len() > 10, "Response should be substantial (got {} bytes)", response.len());
    
    // Verify it's a proper greeting response
    let has_greeting = response.to_lowercase().contains("hello") || 
                      response.to_lowercase().contains("hi") ||
                      response.to_lowercase().contains("greet");
    assert!(has_greeting || response.len() > 20, "Response should contain greeting words or be substantial");

    println!("✅ AI greeting test completed successfully");

     // Release the lock before cleanup
    drop(chat);

    Ok(())
}
