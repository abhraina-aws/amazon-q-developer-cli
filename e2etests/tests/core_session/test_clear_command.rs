#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "clear", feature = "sanity"))]
fn test_clear_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /clear command... | Description: Tests the <code> /clear</code> command to clear conversation history and verify that previous context is no longer remembered by the AI");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro Chat session started");
    
    // Send initial message
    println!("\n🔍 Sending prompt: 'My name is TestUser'");
    let _initial_response = chat.execute_command_with_timeout("My name is TestUser",Some(1000))?;
    println!("📝 Initial response: {} bytes", _initial_response.len());
    println!("📝 INITIAL RESPONSE OUTPUT:");
    println!("{}", _initial_response);
    println!("📝 END INITIAL RESPONSE");
    
    // Execute clear command
    println!("\n🔍 Executing command: '/clear'");
    let _clear_response = chat.execute_command_with_timeout("/clear",Some(1000))?;
    
    // Check if AI remembers previous conversation
    println!("\n🔍 Sending prompt: 'What is my name?'");
    let test_response = chat.execute_command_with_timeout("What is my name?",Some(1000))?;
    println!("📝 Test response: {} bytes", test_response.len());
    println!("📝 TEST RESPONSE OUTPUT:");
    println!("{}", test_response);
    println!("📝 END TEST RESPONSE");
    
    // Verify history is cleared - AI shouldn't remember the name
    assert!(!test_response.to_lowercase().contains("testuser"), "Clear command failed - AI still remembers previous conversation");
    println!("✅ Clear command successful - Conversation history cleared.");
    
    // Release the lock
    drop(chat);
    
    Ok(())
}