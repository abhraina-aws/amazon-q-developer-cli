#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[allow(dead_code)]
fn clean_terminal_output(input: &str) -> String {
    input.replace("(B", "")
}

#[test]
#[cfg(all(feature = "help", feature = "sanity"))]
fn test_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /help command... | Description: Tests the <code> /help</code> command to display all available commands and verify core functionality like quit, clear, tools, and help commands are present");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro-cli Chat session started");

    let response = chat.execute_command_with_timeout("/help",Some(100))?;

    println!("📝 Help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify help content
    assert!(response.contains("Commands"), "Missing Commands section");

    assert!(response.contains("quit"), "Missing quit command");
    assert!(response.contains("clear"), "Missing clear command");
    assert!(response.contains("tools"), "Missing tools command");
    assert!(response.contains("help"), "Missing help command");

    // Verify specific useful commands
    assert!(response.contains("context"), "Missing context management command");
    assert!(response.contains("agent"), "Missing agent management command");
    assert!(response.contains("model"), "Missing model selection command");

    println!("✅ All help content verified!");

    // Release the lock
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "help", feature = "sanity"))]
fn test_multiline_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing multiline input... | Description: Tests <code>ctrl+J multiline </code>command input with embedded newlines");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro-cli Chat session started");

    // Ctrl+J produces ASCII Line Feed (0x0A)
    let ctrl_j = "\x0A";
    let multiline_input = format!("what is aws explain in 100 words.{}what is AI explain in 100 words", ctrl_j);
    let response = chat.execute_command_with_timeout(&multiline_input,Some(1000))?;

    println!("📝 Response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("AWS"), "Response should contain 'AWS'");
    assert!(response.contains("AI"), "Response should contain 'AI'");
    assert!(!response.is_empty(), "Response should not be empty");
    println!("✅ Multiline input processed successfully");

    drop(chat);
    Ok(())
}

#[test]
#[cfg(all(feature = "help", feature = "sanity"))]
fn test_whoami_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing !whoami command... | Description: Tests the <code> !whoami </code> command to display the current user");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro Chat session started");

    let response = chat.execute_command_with_timeout("!whoami",Some(100))?;

    println!("📝 Help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify whoami content
    assert!(!response.is_empty(), "Empty response from whoami command");

    // Verify response contains user information
    assert!(response.len() > 0, "Response should contain user information");

    println!("✅ All whoami command functionality verified!");

    // Release the lock
    drop(chat);
    Ok(())
}

#[test]
#[cfg(all(feature = "help", feature = "sanity"))]
fn test_ctrls_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing ctrl+s input... | Description: Tests <code>ctrl+s</code> command to display available commands in an interactive menu and verify core commands are accessible");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro-cli Chat session started");

    // Ctrl+J produces ASCII Line Feed (0x0A)
    let ctrl_j = "\x13";
    let response = chat.execute_command_with_timeout(ctrl_j,Some(2000))?;
    
    let cleaned_response = clean_terminal_output(&response);

    println!("📝 Response: {} bytes", cleaned_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", cleaned_response);
    println!("📝 END OUTPUT");

    assert!(cleaned_response.contains("agent"),"Response should contain /agent");
    assert!(cleaned_response.contains("clear"),"Response should contain /clear");
    // assert!(cleaned_response.contains("context"),"Response should contain /context");
    // assert!(cleaned_response.contains("code"),"Response should contain /code");
    assert!(cleaned_response.contains("changelog"),"Response should contain /changelog");

    //pressing esc button to close ctrl+s window
    let _esc = chat.execute_command("\x1B")?;

    println!("✅ Ctrl+s input processed successfully");

    drop(chat);
    Ok(())
}

#[test]
#[cfg(all(feature = "help", feature = "sanity"))]
fn test_multiline_with_alt_enter_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing Alt(⌥) + Enter(⏎)  input... | Description: Tests <code>Alt(⌥) + Enter(⏎) </code>for multiline input");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro-cli Chat session started");

    let alt_enter = "\x1B\x0A";
    let aws_prompt = "what is AWS explain in 100 words ";
    let ai_rompt = "what is AI explain in 100 words";

    let combined = format!("{}{}{}", aws_prompt, alt_enter,ai_rompt);
    let response = chat.execute_command_with_timeout(&combined,Some(1000))?;

    println!("📝 Response: {} bytes", response.len());
    println!("📝 FULL OUTPUT: {}",response);
    println!("📝 END");

    assert!(response.contains("AWS"), "Response should contain 'AWS'");
    assert!(response.contains("AI"), "Response should contain 'AI'");
    assert!(!response.is_empty(), "Response should not be empty");
    
    println!("✅ Alt+Enter multiline input processed successfully");

    drop(chat);
    Ok(())
}