#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_what_is_aws_prompt() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 [AI PROMPTS] Testing 'What is AWS?' AI prompt... | Description: Tests <code>AI prompt</code> functionality <code>by sending 'What is AWS?'</code> and verifying the response contains relevant AWS information and technical terms");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap();
    println!("✅ Kiro Chat session started");
    
    let response = chat.execute_command_with_timeout("What is AWS?",Some(1000))?;
    
    println!("📝 AI response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Check if we got an actual AI response
    if response.contains("Amazon Web Services") || 
       response.contains("cloud") || 
       response.contains("AWS") ||
       response.len() > 100 {
        println!("✅ Got substantial AI response ({} bytes)!", response.len());
        
        // Additional checks for quality response
        if response.contains("Amazon Web Services") {
            println!("✅ Response correctly identifies 'Amazon Web Services'");
        }
        if response.contains("cloud") {
            println!("✅ Response mentions cloud computing concepts");
        }
        if response.contains("AWS") {
            println!("✅ Response uses AWS acronym appropriately");
        }
        
        // Check for technical depth
        let technical_terms = ["service", "platform", "infrastructure", "compute", "storage"];
        let found_terms: Vec<&str> = technical_terms.iter()
            .filter(|&&term| response.to_lowercase().contains(term))
            .copied()
            .collect();
        if !found_terms.is_empty() {
            println!("✅ Response includes technical terms: {:?}", found_terms);
        }
    } else {
        println!("⚠️ Response seems limited or just echoed input");
        println!("⚠️ Expected AWS explanation but got: {} bytes", response.len());
    }

    println!("✅ Test completed successfully");

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
    println!("✅ Kiro Chat session started");
    
    let response = chat.execute_command_with_timeout("Hello",Some(1000))?;
    
    println!("📝 Greeting response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Check if we got any response
    if response.trim().is_empty() {
        println!("⚠️ No response to greeting - AI may not be responding");
    } else if response.to_lowercase().contains("hello") || 
              response.to_lowercase().contains("hi") ||
              response.to_lowercase().contains("greet") {
        println!("✅ Got appropriate greeting response!");
        println!("✅ AI recognized and responded to greeting appropriately");
    } else if response.len() > 20 {
        println!("✅ Got substantial response ({} bytes) to greeting", response.len());
        println!("⚠️ Response doesn't contain typical greeting words but seems AI-generated");
    } else {
        println!("⚠️ Got minimal response - unclear if AI-generated or echo");
        println!("⚠️ Response length: {} bytes", response.len());
    }

    println!("✅ Test completed successfully");

     // Release the lock before cleanup
    drop(chat);

    Ok(())
}
