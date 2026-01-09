#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "hooks", feature = "sanity"))]
fn test_hooks_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /hooks command... | Description: Tests the <code> /hooks</code> command to display configured hooks or show no hooks message when none are configured");
    
    let session = q_chat_helper::get_chat_session();
   let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/hooks",Some(500))?;
    
    println!("📝 Hooks command response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify no hooks configured message
    assert!(response.contains("No hooks"), "Missing no hooks configured message");
    
    println!("✅ All hooks command functionality verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "hooks", feature = "sanity"))]
fn test_hooks_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /hooks --help command... | Description: Tests the <code> /hooks --help</code> command to display comprehensive help information for hooks functionality and configuration");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/hooks --help",Some(500))?;
    
    println!("📝 Hooks help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify Usage section
    assert!(response.contains("Usage"), "Missing Usage section");
    assert!(response.contains("/hooks"), "Missing /hooks command in usage section");
    
    // Verify Options section
    assert!(response.contains("Options"), "Missing Options section");
    
    // Verify help flags
    assert!(response.contains("-h") &&  response.contains("--help"), "Missing -h, --help flags");
    
    println!("✅ All hooks help content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "hooks", feature = "sanity"))]
fn test_hooks_h_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /hooks -h command... | Description: Tests the <code> /hooks -h</code> command (short form) to display hooks help information and verify flag handling");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/hooks -h",Some(500))?;
    
    println!("📝 Hooks help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify Usage section
    assert!(response.contains("Usage"), "Missing Usage section");
    assert!(response.contains("/hooks"), "Missing /hooks command in usage section");
    
    // Verify Options section
    assert!(response.contains("Options"), "Missing Options section");
    
    // Verify help flags
    assert!(response.contains("-h") &&  response.contains("--help"), "Missing -h, --help flags");
    
    println!("✅ All hooks help content verified!");
    
    drop(chat);

    Ok(())
}