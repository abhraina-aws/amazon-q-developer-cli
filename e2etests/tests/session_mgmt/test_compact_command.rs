#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_compact_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /compact command... | Description: Tests the <code>/compact</code> command to compress conversation history and verify successful compaction or appropriate messaging for short conversations");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
     
    let response = chat.execute_command_with_timeout("What is AWS explain 100 charectors",Some(2000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("/compact",Some(2000))?;
    
    println!("📝 Compact response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify compact response - either success or too short
    let has_success = response.contains("history") && response.contains("compacted") && response.contains("successfully");
    let has_short_msg = response.contains("Conversation") && response.contains("short");
    assert!(has_success || has_short_msg, "Expected compact success message or conversation too short message");
    
    println!("✅ All compact content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_compact_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /compact --help command... | Description: Tests the <code> /compact --help</code> command to display comprehensive help information for conversation compaction functionality");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/compact --help",Some(2000))?;
    
    println!("📝 Compact help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify Usage section
    assert!(response.contains("Usage"), "Missing usage format");
    
    // Verify Arguments section
    assert!(response.contains("Arguments"), "Missing Arguments section");
    
    // Verify Options section
    assert!(response.contains("Options:"), "Missing Options section");
    assert!(response.contains("--show-summary"), "Missing --show-summary option");
    assert!(response.contains("--messages-to-exclude"), "Missing --messages-to-exclude option");
    assert!(response.contains("--truncate-large-messages"), "Missing --truncate-large-messages option");
    assert!(response.contains("--max-message-length"), "Missing --max-message-length option");
    assert!(response.contains("-h") &&  response.contains("--help"), "Missing -h, --help flags");
    
    println!("✅ All compact help content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_compact_h_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /compact -h command... | Description: Tests the <code> /compact -h</code> command (short form) to display compact help information");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/compact -h",Some(2000))?;
    
    println!("📝 Compact help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify Usage section
    assert!(response.contains("Usage"), "Missing usage format");
    
    // Verify Arguments section
    assert!(response.contains("Arguments"), "Missing Arguments section");
    
    // Verify Options section
    assert!(response.contains("Options:"), "Missing Options section");
    assert!(response.contains("--show-summary"), "Missing --show-summary option");
    assert!(response.contains("--messages-to-exclude"), "Missing --messages-to-exclude option");
    assert!(response.contains("--truncate-large-messages"), "Missing --truncate-large-messages option");
    assert!(response.contains("--max-message-length"), "Missing --max-message-length option");
    assert!(response.contains("-h") &&  response.contains("--help"), "Missing -h, --help flags");
    
    println!("✅ All compact help content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_compact_truncate_true_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing /compact --truncate-large-messages true command... | Description: Test that the <code> /compact  —truncate-large-messages true</code> truncates large messages");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
     
    let response = chat.execute_command_with_timeout("What is AWS explain 100 chaarectors",Some(3000))?;
    
    println!("📝 AI response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("/compact --truncate-large-messages true",Some(3000))?;
    
    println!("📝 Compact response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    let has_truncating = response.to_lowercase().contains("truncating");
    let has_success = response.contains("history") && response.contains("compacted") && response.contains("successfully");
    let has_short_msg = response.contains("Conversation") && response.contains("short");
    assert!(has_truncating || has_success || has_short_msg, "Expected truncation message, compact success, or conversation too short message");
    
    println!("✅ All compact content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_compact_truncate_false_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing /compact --truncate-large-messages false command... | Description: Tests the <code> /compact --truncate-large-messages false</code> command to verify no message truncation occurs");
    
    let session = q_chat_helper::get_chat_session();
     let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
     
    let response = chat.execute_command_with_timeout("What is AWS explain 100 chaarectors",Some(3000))?;
    
    println!("📝 AI response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("/compact --truncate-large-messages false",Some(3000))?;
    
    println!("📝 Compact response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify compact response - either success or too short
    let has_success = response.contains("history") && response.contains("compacted") && response.contains("successfully");
    let has_short_msg = response.contains("Conversation") && response.contains("short");
    assert!(has_success || has_short_msg, "Expected compact success message or conversation too short message");
    
    println!("✅ All compact content verified!");
    
    drop(chat);

    Ok(())
}


#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_show_summary() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing /compact --show-summary command... | Description: Tests the <code> /compact --show-summary</code> command to display conversation summary after compaction");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("What is AWS explain 100 chaarectors",Some(2000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("What is AWS explain 100 chaarectors",Some(2000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("/compact --show-summary",Some(3000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify compact response - either success or too short
    let has_success = response.contains("history") && response.contains("compacted") && response.contains("successfully");
    let has_short_msg = response.contains("Conversation") && response.contains("short");
    assert!(has_success || has_short_msg, "Expected compact success message or conversation too short message");
    
    // Verify compact sumary response
    assert!(response.to_lowercase().contains("conversation") && response.to_lowercase().contains("summary"), "Missing Summary section");
    println!("✅ All compact content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_max_message_truncate_true() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing /compact --truncate-large-messages true --max-message-length command... | Description: Test <code> /compact --truncate-large-messages true  --max-message-length <MAX_MESSAGE_LENGTH></code> command compacts the conversation by summarizing it to free up context space, truncating large messages to a maximum of provided <MAX_MESSAGE_LENGTH>. ");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let prompt_one_response = chat.execute_command_with_timeout("What is AWS explain 50 words",Some(2000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", prompt_one_response);
    println!("📝 END OUTPUT");

    let prompt_two_response = chat.execute_command_with_timeout("What is DL explain in 50 words",Some(2000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", prompt_two_response);
    println!("📝 END OUTPUT");

    let truncate_response = chat.execute_command_with_timeout("/compact --truncate-large-messages true  --max-message-length 5",Some(3000))?;
    
   
    println!("📝 FULL OUTPUT:");
    println!("{}", truncate_response);
    println!("📝 END OUTPUT");
    
    // Verify compact response - either success or too short
    let has_truncating = truncate_response.to_lowercase().contains("truncating");
    let has_success = truncate_response.contains("history") && truncate_response.contains("compacted") && truncate_response.contains("successfully");
    let has_short_msg = truncate_response.contains("Conversation") && truncate_response.contains("short");
    assert!(has_truncating || has_success || has_short_msg, "Expected truncation message, compact success, or conversation too short message");
    
    // Verify compact sumary response
    assert!(truncate_response.to_lowercase().contains("conversation") && truncate_response.to_lowercase().contains("summary"), "Missing Summary section");
    println!("✅ All compact content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_max_message_truncate_false() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing /compact --truncate-large-messages false --max-message-length command... | Description: Test <code> /compact --truncate-large-messages false --max-message-length <MAX_MESSAGE_LENGTH></code> command compacts the conversation by summarizing it to free up context space, but keeps large messages intact (no truncation) despite the max-message-length setting.");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let prompt_one_response = chat.execute_command_with_timeout("What is AWS explain 100 chaarectors",Some(2000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", prompt_one_response);
    println!("📝 END OUTPUT");

    let prompt_two_response = chat.execute_command_with_timeout("What is DL explain in 100 chrectors",Some(2000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", prompt_two_response);
    println!("📝 END OUTPUT");

    let truncate_response = chat.execute_command_with_timeout("/compact --truncate-large-messages false  --max-message-length 5",Some(1000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", truncate_response);
    println!("📝 END OUTPUT");
    
    // Verify compact response - either success or too short
    let has_success = truncate_response.contains("history") && truncate_response.contains("compacted") && truncate_response.contains("successfully");
    let has_short_msg = truncate_response.contains("Conversation") && truncate_response.contains("short");
    assert!(has_success || has_short_msg, "Expected compact success message or conversation too short message");
    
    // Verify compact sumary response
    assert!(truncate_response.to_lowercase().contains("conversation") && truncate_response.to_lowercase().contains("summary"), "Missing Summary section");
    println!("✅ All compact content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_max_message_length_invalid() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing /compact --max-message-length command... | Description: Tests the <code> /compact --max-message-length <MAX_MESSAGE_LENGTH></code> command with invalid subcommand to verify proper error handling and help display");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    let response = chat.execute_command_with_timeout("What is AWS explain 100 chaarectors",Some(2000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("What is DL explain in 100 chrectors",Some(2000))?;
    
    println!("📝 AI response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("/compact --max-message-length 5",Some(2000))?;
    
    println!("📝 Compact response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify error message for missing required argument
    assert!(response.contains("error"), "Missing error message");
    assert!(response.contains("--truncate-large-messages") && response.contains("<TRUNCATE_LARGE_MESSAGES>") && response.contains("--max-message-length") && response.contains("<MAX_MESSAGE_LENGTH>"), "Missing required argument info");
    assert!(response.contains("Usage"), "Missing usage info");
    assert!(response.contains("--help"), "Missing help suggestion");

    println!("✅ All compact content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_compact_messages_to_exclude_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /compact command... | Description: Test <code> /compact --messages-to-exclude <MESSAGES_TO_EXCLUDE></code> command compacts the conversation by summarizing it to free up context space, excluding provided number of user-assistant message pair from the summarization process.");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
     
    let response = chat.execute_command_with_timeout("What is AWS explain 100 chaarectors",Some(2000))?;
    
    println!("📝 AI response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("What is fibonacci explain in 100 charectors?",Some(2000))?;
    
    println!("📝 AI response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("/compact --messages-to-exclude 1",Some(2000))?;
    
    println!("📝 Compact response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify compact response - either success or too short
    let has_success = response.contains("history") && response.contains("compacted") && response.contains("successfully");
    let has_short_msg = response.contains("Conversation") && response.contains("short");
    assert!(has_success || has_short_msg, "Expected compact success message or conversation too short message");
    
    println!("✅ All compact content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "compact", feature = "sanity"))]
fn test_compact_messages_to_exclude_show_sumary_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /compact command... | Description: Test <code> /compact --messages-to-exclude <MESSAGES_TO_EXCLUDE> --show-summary</code> command compacts the conversation by summarizing it to free up context space, excluding provided number of user-assistant message pair from the summarization process and prints the coversation summary.");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    chat.execute_command_with_timeout("/clear",Some(2000))?;
    chat.execute_command("y")?;
    let response = chat.execute_command_with_timeout("What is AWS explain 100 chaarectors",Some(2000))?;
    
    println!("📝 AI response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("What is fibonacci explain in 100 charectors",Some(2000))?;
    
    println!("📝 AI response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    let response = chat.execute_command_with_timeout("/compact --messages-to-exclude 1 --show-summary",Some(2000))?;
    
    println!("📝 Compact response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify compact response - either success or too short
    let has_success = response.contains("history") && response.contains("compacted") && response.contains("successfully");
    let has_short_msg = response.contains("Conversation") && response.contains("short");
    assert!(has_success || has_short_msg, "Expected compact success message or conversation too short message");
    
    // Verify compact sumary response
    assert!(response.to_lowercase().contains("conversation") && response.to_lowercase().contains("summary"), "Missing Summary section");

    // Verify messages got excluded
    assert!(!response.to_lowercase().contains("fibonacci"), "Fibonacci should not be present in compact response");

    println!("✅ All compact content verified!");
    
    drop(chat);

    Ok(())
}